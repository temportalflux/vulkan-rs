use crate::{
	ecs::{
		self,
		components::{BoidRender, Orientation, Position2D, Velocity2D},
		NamedSystem,
	},
	engine::{
		input,
		math::{vector, Quaternion},
		rand::{self, Rng},
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
		static SPAWN_COUNT: usize = 10;
		static MAX_POS_ABS: f32 = 13.0;
		static SPAWN_VELOCITY: f32 = 2.0;
		if input::System::read()
			.is_key_pressed(input::KeyCode::Equals, std::time::Duration::from_millis(1))
		{
			let mut rng = rand::thread_rng();
			for entity_props in (0..SPAWN_COUNT).map(|_| {
				let angle = rng.gen_range(0.0..360.0_f32).to_radians();
				(
					vector![
						rng.gen_range(-MAX_POS_ABS..MAX_POS_ABS),
						rng.gen_range(-MAX_POS_ABS..MAX_POS_ABS)
					],
					Quaternion::from_axis_angle(-world::global_forward(), angle),
					Quaternion::from_axis_angle(-world::global_forward(), angle)
						.rotate(&world::global_up())
						.subvec::<2>(None) * -SPAWN_VELOCITY,
					vector![0.5, 0.0, 1.0, 1.0],
				)
			}) {
				entities
					.build_entity()
					.with(Position2D(entity_props.0), &mut pos)
					.with(Orientation(entity_props.1), &mut orientation)
					.with(Velocity2D(entity_props.2), &mut velocity)
					.with(BoidRender::new(entity_props.3), &mut boid_render)
					.build();
			}
		}
	}
}
