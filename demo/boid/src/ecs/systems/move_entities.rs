use crate::{
	ecs::{
		self,
		components::{Orientation, Position2D, Velocity2D},
		resources::DeltaTime,
		NamedSystem,
	},
	engine::{math::Quaternion, world},
};

pub struct MoveEntities {}

impl Default for MoveEntities {
	fn default() -> MoveEntities {
		MoveEntities {}
	}
}

impl NamedSystem for MoveEntities {
	fn name() -> &'static str {
		"move_entities"
	}
}

impl<'a> ecs::System<'a> for MoveEntities {
	type SystemData = (
		ecs::Read<'a, DeltaTime>,
		ecs::WriteStorage<'a, Position2D>,
		ecs::ReadStorage<'a, Velocity2D>,
		ecs::WriteStorage<'a, Orientation>,
	);

	fn run(
		&mut self,
		(delta_time, mut position_store, velocity_store, mut orientation_store): Self::SystemData,
	) {
		use ecs::Join;
		use std::ops::DerefMut;

		let angle = 90.0_f32.to_radians();
		let dt = delta_time.get().as_secs_f32();
		for (position, velocity, orientation) in
			(&mut position_store, &velocity_store, &mut orientation_store).join()
		{
			*position.deref_mut() += velocity.get() * dt;
			orientation.rotate(Quaternion::from_axis_angle(
				-world::global_forward(),
				angle * dt,
			));
		}
	}
}
