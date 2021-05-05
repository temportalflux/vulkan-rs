use crate::{
	ecs,
	engine::{math::Quaternion, world},
	graphics,
};
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

		let mut instances = Vec::new();
		for (pos, renderable) in (&pos, &renderable).join() {
			instances.push(
				graphics::Instance::default()
					.with_pos(pos.0.subvec::<3>(None))
					.with_orientation(Quaternion::from_axis_angle(
						-world::global_forward(),
						90.0_f32.to_radians(),
					))
					.with_color(renderable.color),
			);
		}

		let mut render_boids = self.renderer.write().unwrap();
		render_boids.set_instances(instances).unwrap();
	}
}
