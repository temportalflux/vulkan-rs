use crate::{
	ecs::{
		self,
		components::{ai::Wander2D, BoidRender, Orientation, Position2D, Velocity2D},
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
		ecs::WriteStorage<'a, Wander2D>,
	);
	fn run(
		&mut self,
		(
			entities,
			mut store_position,
			mut store_velocity,
			mut store_orientation,
			mut store_boid_render,
			mut store_wander,
		): Self::SystemData,
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
					.with(Position2D(entity_props.0), &mut store_position)
					.with(Orientation(entity_props.1), &mut store_orientation)
					.with(Velocity2D(entity_props.2), &mut store_velocity)
					.with(BoidRender::new(entity_props.3), &mut store_boid_render)
					.with(Wander2D::default().with_speed(2.0), &mut store_wander)
					.build();
			}
		}
	}
}
