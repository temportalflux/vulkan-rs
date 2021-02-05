#pragma once

#include "CoreInclude.hpp"

#include "evcs/types.h"
#include "network/NetworkCore.hpp"
#include "physics/PhysicsController.hpp"
#include "world/Events.hpp"
#include "world/WorldCoordinate.hpp"

NS_EVCS
FORWARD_DEF(NS_SYSTEM, class PhysicsIntegration);
FORWARD_DEF(NS_SYSTEM, class IntegratePlayerPhysics);
NS_END
FORWARD_DEF(NS_PHYSICS, class Material);
FORWARD_DEF(NS_PHYSICS, class RigidBody);
FORWARD_DEF(NS_PHYSICS, class Scene);
FORWARD_DEF(NS_PHYSICS, class System);
FORWARD_DEF(NS_PHYSICS, class ChunkCollisionManager);
FORWARD_DEF(NS_WORLD, class Terrain);

NS_WORLD

using DimensionId = ui32;

class World : public virtual_enable_shared_from_this<World>
{

public:
	World();
	virtual ~World();

	BroadcastDelegate<void(f32 const& deltaTime)> onSimulate;

	std::shared_ptr<physics::Material> playerPhysicsMaterial();
	std::shared_ptr<physics::Scene> dimensionScene(DimensionId dimId);
	std::shared_ptr<world::Terrain> terrain(DimensionId dimId);
	void addTerrainEventListener(ui32 dimId, std::shared_ptr<WorldEventListener> listener);
	void removeTerrainEventListener(ui32 dimId, std::shared_ptr<WorldEventListener> listener);

	virtual void init();
	virtual bool shouldConnectToPhysxDebugger() const { return true; }
	virtual void uninit();
	void update(f32 deltaTime);

	/**
	 * Creates a player entity (though doesn't include any rendering or POV components/views).
	 * Returns the EVCS entity id.
	 */
	evcs::Identifier createPlayer(ui32 clientNetId, world::Coordinate const& position);
	void destroyPlayer(ui32 userNetId, evcs::Identifier entityId);

	void createPlayerController(network::Identifier userNetId, evcs::Identifier localEntityId);
	bool hasPhysicsController(network::Identifier userNetId) const;
	physics::Controller& getPhysicsController(network::Identifier userNetId);
	void destroyPlayerController(network::Identifier userNetId);

	virtual void loadChunk(DimensionId const& dimId, math::Vector3Int const& coord) {}

	f32 const& simulationFrequency() const;

protected:
	struct Dimension
	{
		ui32 id;
		std::shared_ptr<physics::Scene> mpScene;
		std::shared_ptr<world::Terrain> mpTerrain;
		std::shared_ptr<physics::ChunkCollisionManager> mpChunkCollisionManager;
	};
	Dimension& dimension(DimensionId const& dimId);

private:

	std::shared_ptr<physics::System> mpPhysics;
	std::shared_ptr<physics::Material> mpPlayerPhysicsMaterial;
	std::shared_ptr<evcs::system::PhysicsIntegration> mpSystemPhysicsIntegration;
	std::shared_ptr<evcs::system::IntegratePlayerPhysics> mpSystemIntegratePlayerPhysics;
	void initializePhysics();
	void uninitializePhysics();

	Dimension mOverworld;
	void createDimension(Dimension *dim);
	void destroyDimension(Dimension *dim);

	std::map<ui32, physics::Controller> mPhysicsControllerByUserNetId;

	f32 mSimulationFrequency;
	f32 mTimeSinceLastSimulate;

};

NS_END
