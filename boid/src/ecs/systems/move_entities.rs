use crate::{
	ecs::{self, components::Orientation, resources::DeltaTime},
	engine::{math::Quaternion, world},
};

pub struct MoveEntities {}

impl Default for MoveEntities {
	fn default() -> MoveEntities {
		MoveEntities {}
	}
}

impl<'a> ecs::System<'a> for MoveEntities {
	type SystemData = (ecs::Read<'a, DeltaTime>, ecs::WriteStorage<'a, Orientation>);

	fn run(&mut self, (delta_time, mut orientation_store): Self::SystemData) {
		use ecs::Join;
		let angle = 90.0_f32.to_radians();
		for orientation in (&mut orientation_store).join() {
			orientation.rotate(Quaternion::from_axis_angle(
				-world::global_forward(),
				angle * delta_time.get().as_secs_f32(),
			));
		}
	}
}