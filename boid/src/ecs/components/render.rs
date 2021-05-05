use crate::{
	ecs::{Component, VecStorage},
	engine::math::Vector,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidRender {
	pub color: Vector<f32, 4>,
}

impl Component for BoidRender {
	type Storage = VecStorage<Self>;
}

impl BoidRender {
	pub fn new(color: Vector<f32, 4>) -> BoidRender {
		BoidRender { color }
	}
}
