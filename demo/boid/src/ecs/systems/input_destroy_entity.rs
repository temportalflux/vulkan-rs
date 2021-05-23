use crate::{ecs::{self, NamedSystem}, engine::input};

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
	type SystemData = ();
	fn run(&mut self, _: Self::SystemData) {
		if input::System::read().is_key_pressed(input::KeyCode::Minus, std::time::Duration::from_millis(1)) {
			log::debug!("destroy entity");
		}
	}
}

