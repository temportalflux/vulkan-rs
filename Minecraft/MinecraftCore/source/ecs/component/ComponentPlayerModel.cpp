#include "ecs/component/ComponentPlayerModel.hpp"

#include "asset/TypedAssetPath.hpp"
#include "asset/ModelAsset.hpp"
#include "render/model/SkinnedModelManager.hpp"
#include "render/EntityInstanceBuffer.hpp"

using namespace ecs;
using namespace ecs::component;

static asset::TypedAssetPath<asset::Model> PLAYER_MODEL_PATH = asset::TypedAssetPath<asset::Model>::Create(
	"assets/models/DefaultHumanoid/DefaultHumanoid.te-asset"
);

DEFINE_ECS_COMPONENT_STATICS(PlayerModel)

PlayerModel::PlayerModel() : Component()
{
}

PlayerModel::~PlayerModel()
{
	this->mModelHandle.destroy();
	this->mInstanceHandle.destroy();
}

PlayerModel& PlayerModel::createModel(std::shared_ptr<graphics::SkinnedModelManager> modelManager)
{
	// Create a skinned model instance of the player model
	this->mModelHandle = modelManager->createHandle();
	modelManager->setModel(this->mModelHandle, PLAYER_MODEL_PATH.load(asset::EAssetSerialization::Binary));
	return *this;
}

DynamicHandle<graphics::SkinnedModel> const& PlayerModel::modelHandle() const { return this->mModelHandle; }

PlayerModel& PlayerModel::createInstance(std::shared_ptr<graphics::EntityInstanceBuffer> instanceBuffer)
{
	this->mInstanceHandle = instanceBuffer->createHandle();
	return *this;
}

DynamicHandle<graphics::EntityInstanceData> const& PlayerModel::instanceHandle() const { return this->mInstanceHandle; }

PlayerModel& PlayerModel::setTextureId(std::string const& textureId)
{
	this->mTextureId = textureId;
	return *this;
}

std::string const& PlayerModel::textureId() const { return this->mTextureId; }
