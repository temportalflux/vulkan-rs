use crate::{engine::{EngineSystem}, ecs::{self}};

pub struct Context<'a, 'b> {
	pub world: ecs::World,
	pub dispatcher: ecs::Dispatcher<'a, 'b>,
}

impl<'a, 'b> EngineSystem for Context<'a, 'b> {
	fn update(&mut self, delta_time: std::time::Duration) {
		{
			use ecs::WorldExt;
			let mut resource = self.world.write_resource::<ecs::resources::DeltaTime>();
			*resource = ecs::resources::DeltaTime(delta_time);
		}
		self.dispatcher.dispatch(&mut self.world);
	}
}
