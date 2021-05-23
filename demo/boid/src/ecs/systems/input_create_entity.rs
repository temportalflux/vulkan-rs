use crate::{ecs::{self, NamedSystem}, engine::input};

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
	type SystemData = ();
	fn run(&mut self, _: Self::SystemData) {
		if input::System::read().is_key_pressed(input::KeyCode::Equals, std::time::Duration::from_millis(1)) {
			log::debug!("spawn entity");
		}
	}
}
