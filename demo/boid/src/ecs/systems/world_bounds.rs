use crate::{
	ecs::{self, components::Position2D, NamedSystem},
	engine::math::Vector,
};

pub struct WorldBounds {
	min: Vector<f32, 2>,
	max: Vector<f32, 2>,
}

impl Default for WorldBounds {
	fn default() -> WorldBounds {
		WorldBounds {
			min: Vector::filled(0.0),
			max: Vector::filled(0.0),
		}
	}
}

impl NamedSystem for WorldBounds {
	fn name() -> &'static str {
		"world_bounds"
	}
	fn dependencies(&self) -> Vec<&'static str> {
		vec![ecs::systems::MoveEntities::name()]
	}
}

impl WorldBounds {
	pub fn with_bounds(mut self, min: Vector<f32, 2>, max: Vector<f32, 2>) -> Self {
		self.min = min;
		self.max = max;
		self
	}
}

impl<'a> ecs::System<'a> for WorldBounds {
	type SystemData = ecs::WriteStorage<'a, Position2D>;

	fn run(&mut self, mut position_store: Self::SystemData) {
		use ecs::Join;
		use std::ops::DerefMut;
		for position in (&mut position_store).join() {
			let pos = position.deref_mut();
			for i in 0..2 {
				let range = self.max[i] - self.min[i];
				while pos[i] >= self.max[i] {
					pos[i] -= range;
				}
				while pos[i] < self.min[i] {
					pos[i] += range;
				}
			}
		}
	}
}
