use crate::{ecs, graphics};
use std::sync::{Arc, RwLock};

pub struct InstanceCollector {
	renderer: Arc<RwLock<graphics::RenderBoids>>,
}

impl InstanceCollector {
	pub fn new(renderer: Arc<RwLock<graphics::RenderBoids>>) -> InstanceCollector {
		InstanceCollector { renderer }
	}
}

impl<'a> ecs::System<'a> for InstanceCollector {
	type SystemData = (
		ecs::ReadStorage<'a, ecs::components::Position2D>,
		ecs::ReadStorage<'a, ecs::components::BoidRender>,
	);

	fn run(&mut self, (pos, renderable): Self::SystemData) {
		use ecs::Join;
		let _render_boids = self.renderer.write().unwrap();
		let _count = renderable.count();
		for (pos, _renderable) in (&pos, &renderable).join() {
			log::debug!("Found pos {:?}", pos);
		}
	}
}
