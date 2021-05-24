use crate::{
	ecs::{self, NamedSystem},
	engine::{
		input,
		rand::{self, Rng},
	},
};

pub struct InputDestroyEntity {}

impl Default for InputDestroyEntity {
	fn default() -> Self {
		Self {}
	}
}

impl NamedSystem for InputDestroyEntity {
	fn name() -> &'static str {
		"input_destroy_entity"
	}
}

impl<'a> ecs::System<'a> for InputDestroyEntity {
	type SystemData = ecs::Entities<'a>;
	fn run(&mut self, entities: Self::SystemData) {
		use ecs::Join;
		static DESTROY_COUNT: usize = 10;
		if input::System::read()
			.is_key_pressed(input::KeyCode::Minus, std::time::Duration::from_millis(1))
		{
			let mut rng = rand::thread_rng();
			let ent_count = entities.join().count();

			let mut ent_indices = Vec::new();
			if ent_count <= DESTROY_COUNT {
				ent_indices = (0..ent_count).collect();
			} else {
				while ent_indices.len() < DESTROY_COUNT {
					let next_index = rng.gen_range(0..ent_count);
					if !ent_indices.contains(&next_index) {
						ent_indices.push(next_index);
					}
				}
			}

			ent_indices.sort();

			let mut consumed_sized = 0;
			let mut ent_iter = entities.join();
			for ent_index in ent_indices {
				let next_index = ent_index - consumed_sized;
				consumed_sized = ent_index + 1;
				if let Some(entity) = ent_iter.nth(next_index) {
					entities.delete(entity).unwrap();
				}
			}
		}
	}
}
