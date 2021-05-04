use crate::{ecs};

pub struct InstanceCollector {

}

impl InstanceCollector {
	pub fn new() -> InstanceCollector {
		InstanceCollector {}
	}
}

impl<'a> ecs::System<'a> for InstanceCollector {
	type SystemData = (
		ecs::ReadStorage<'a, ecs::components::Position2D>,
		ecs::ReadStorage<'a, ecs::components::BoidRender>,
	);

	fn run(&mut self, (pos, renderable): Self::SystemData) {
		use ecs::Join;
		let count = renderable.count();
		for (pos, renderable) in (&pos, &renderable).join() {
			log::debug!("Found pos {:?}", pos);
		}
	}
}
