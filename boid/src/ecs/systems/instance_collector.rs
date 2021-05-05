use crate::{
	ecs::{
		self,
		components::{BoidRender, Orientation, Position2D},
	},
	graphics,
};
use std::sync::{Arc, RwLock};

pub struct InstanceCollector {
	expansion_step: usize,
	renderer: Arc<RwLock<graphics::RenderBoids>>,
}

impl InstanceCollector {
	pub fn new(
		renderer: Arc<RwLock<graphics::RenderBoids>>,
		expansion_step: usize,
	) -> InstanceCollector {
		InstanceCollector {
			renderer,
			expansion_step,
		}
	}
}

impl<'a> ecs::System<'a> for InstanceCollector {
	type SystemData = (
		ecs::ReadStorage<'a, Position2D>,
		ecs::ReadStorage<'a, Orientation>,
		ecs::ReadStorage<'a, BoidRender>,
	);

	fn run(&mut self, (pos, orientation, renderable): Self::SystemData) {
		use ecs::Join;

		let mut instances = Vec::new();
		for (pos, orientation, renderable) in (&pos, &orientation, &renderable).join() {
			instances.push(
				graphics::Instance::default()
					.with_orientation(*orientation.get())
					.with_pos(pos.0.subvec::<3>(None))
					.with_color(renderable.color),
			);
		}

		let mut render_boids = self.renderer.write().unwrap();
		render_boids
			.set_instances(instances, self.expansion_step)
			.unwrap();
	}
}
