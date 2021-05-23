use crate::{
	ecs::{
		self,
		components::{BoidRender, Orientation, Position2D, Velocity2D},
		NamedSystem,
	},
	engine::{
		input,
		math::{vector, Quaternion},
		world,
	},
};

pub struct InputCreateEntity {}

impl Default for InputCreateEntity {
	fn default() -> Self {
		Self {}
	}
}

impl NamedSystem for InputCreateEntity {
	fn name() -> &'static str {
		"input_create_entity"
	}
}

impl<'a> ecs::System<'a> for InputCreateEntity {
	type SystemData = (
		ecs::Entities<'a>,
		ecs::WriteStorage<'a, Position2D>,
		ecs::WriteStorage<'a, Velocity2D>,
		ecs::WriteStorage<'a, Orientation>,
		ecs::WriteStorage<'a, BoidRender>,
	);
	fn run(
		&mut self,
		(entities, mut pos, mut velocity, mut orientation, mut boid_render): Self::SystemData,
	) {
		if input::System::read()
			.is_key_pressed(input::KeyCode::Equals, std::time::Duration::from_millis(1))
		{
			entities
				.build_entity()
				.with(ecs::components::Position2D(vector![0.0, 0.0]), &mut pos)
				.with(
					ecs::components::Orientation(Quaternion::from_axis_angle(
						-world::global_forward(),
						360.0_f32.to_radians(),
					)),
					&mut orientation,
				)
				.with(
					ecs::components::Velocity2D(vector![-2.0, 2.0]),
					&mut velocity,
				)
				.with(
					ecs::components::BoidRender::new(vector![1.0, 1.0, 1.0, 1.0]),
					&mut boid_render,
				)
				.build();
		}
	}
}
