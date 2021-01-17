#include "game/GameServer.hpp"

#include "game/GameInstance.hpp"
#include "network/NetworkInterface.hpp"
#include "network/packet/NetworkPacketChatMessage.hpp"
#include "network/packet/NetworkPacketUpdateUserInfo.hpp"
#include "world/World.hpp"

using namespace game;

logging::Logger SERVER_LOG = DeclareLog("Server", LOG_INFO);

Server::Server() : Session()
{
	this->serverRSA().generate();
	this->mServerSettings.readFromDisk();
}

Server::~Server()
{
	Game::networkInterface()->setType(network::EType::eClient).stop();
}

void Server::init()
{
	auto& saveDataRegistry = Game::Get()->saveData();
	auto saveId = this->mServerSettings.saveId();
	auto& saveInstance = saveDataRegistry.has(saveId)
		? saveDataRegistry.get(saveId)
		: saveDataRegistry.create(saveId);
	this->userRegistry().scan(saveInstance.userDirectory());
	game::Game::Get()->createWorld()->init(&saveInstance);
}

void Server::setupNetwork(utility::Flags<network::EType> flags)
{
	auto& networkInterface = *Game::networkInterface();
	networkInterface.onNetworkStarted.bind(this->weak_from_this(), std::bind(
		&Server::onNetworkStarted, this, std::placeholders::_1
	));
	networkInterface.OnDedicatedClientAuthenticated.bind(std::bind(
		&Server::onDedicatedClientAuthenticated, this,
		std::placeholders::_1, std::placeholders::_2
	));
	networkInterface.OnDedicatedClientDisconnected.bind(std::bind(
		&game::Server::onDedicatedClientDisconnected, this,
		std::placeholders::_1, std::placeholders::_2
	));
	networkInterface.onConnectionClosed.bind(std::bind(
		&game::Server::onNetworkConnnectionClosed, this,
		std::placeholders::_1, std::placeholders::_2, std::placeholders::_3
	));
	networkInterface.onNetworkStopped.bind(this->weak_from_this(), std::bind(
		&Server::onNetworkStopped, this, std::placeholders::_1
	));

	networkInterface
		.setType(flags)
		.setAddress(network::Address().setPort(this->mServerSettings.port()));
}

void Server::onNetworkStarted(network::Interface *pInterface)
{
	assert(game::Game::Get()->world());
}

void Server::onNetworkConnectionOpened(network::Interface *pInterface, ui32 connection, ui32 netId)
{
	// both dedicated and integrated servers to create a user for the netId
	this->addConnectedUser(netId);
}

void Server::kick(ui32 netId)
{
	auto* pInterface = Game::networkInterface();
	pInterface->closeConnection(pInterface->getConnectionFor(netId));
}

void Server::onDedicatedClientAuthenticated(network::Interface *pInterface, ui32 netId)
{
	// Tell the newly joined user about all the existing clients
	for (auto const& anyNetId : pInterface->connectedClientNetIds())
	{
		if (anyNetId == netId) continue;
		network::packet::UpdateUserInfo::create()
			->setNetId(anyNetId)
			.setInfo(this->userRegistry().loadInfo(
				this->findConnectedUser(anyNetId)
			))
			.sendTo(netId);
	}

	this->associatePlayer(netId, game::Game::Get()->world()->createPlayer());
}

void Server::onDedicatedClientDisconnected(network::Interface *pInterface, ui32 netId)
{
	assert(pInterface->type().includes(network::EType::eServer));
	if (this->hasConnectedUser(netId))
	{
		auto const& userId = this->findConnectedUser(netId);
		if (userId.isValid())
		{
			auto userInfo = this->userRegistry().loadInfo(userId);
			network::packet::ChatMessage::broadcastServerMessage(
				utility::formatStr("%s has left the server.", userInfo.name().c_str())
			);
			this->destroyPlayer(netId);
		}
	}
}

void Server::onNetworkConnnectionClosed(network::Interface *pInterface, ui32 connection, ui32 netId)
{
	assert(pInterface->type().includes(network::EType::eServer));
	this->removeConnectedUser(netId);
}

void Server::onNetworkStopped(network::Interface *pInterface)
{
	this->clearConnectedUsers();
}

bool Server::hasSaveForUser(utility::Guid const& id) const
{
	return this->userRegistry().contains(id);
}

void Server::initializeUser(utility::Guid const& id, crypto::RSAKey const& key)
{
	auto& registry = this->userRegistry();
	registry.addId(id);
	registry.initializeUser(id, key);
}

crypto::RSAKey Server::getUserPublicKey(utility::Guid const& id) const
{
	return this->userRegistry().loadKey(id);
}

game::UserInfo Server::getUserInfo(utility::Guid const& id) const
{
	return this->userRegistry().loadInfo(id);
}

void Server::associatePlayer(ui32 netId, ecs::Identifier entityId)
{
	SERVER_LOG.log(LOG_INFO, "Linking network-id %u to player entity %u", netId, entityId);
	this->mNetIdToPlayerEntityId.insert(std::make_pair(netId, entityId));
}

void Server::destroyPlayer(ui32 netId)
{
	auto iter = this->mNetIdToPlayerEntityId.find(netId);
	assert(iter != this->mNetIdToPlayerEntityId.end());
	SERVER_LOG.log(LOG_INFO, "Unlinking network-id %u from player entity %u", netId, iter->second);
	game::Game::Get()->world()->destroyPlayer(iter->second);
	this->mNetIdToPlayerEntityId.erase(iter);
}
