use crate::engine::{
	graphics::{self, flags, pipeline},
	math::Vector,
};

pub struct Instance {
	pub pos: Vector<f32, 2>,
	_pos_padding: [f32; 2],
	pub color: Vector<f32, 4>,
}

impl Default for Instance {
	fn default() -> Instance {
		Instance {
			pos: Vector::default(),
			color: Vector::default(),
			_pos_padding: [0.0, 0.0],
		}
	}
}

impl Instance {
	pub fn with_pos(mut self, pos: Vector<f32, 2>) -> Self {
		self.pos = pos;
		self
	}
	pub fn with_color(mut self, color: Vector<f32, 4>) -> Self {
		self.color = color;
		self
	}
}

impl pipeline::vertex::Object for Instance {
	fn attributes() -> Vec<pipeline::vertex::Attribute> {
		vec![
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Instance, pos),
				format: flags::Format::R32G32_SFLOAT,
			},
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Instance, color),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
		]
	}
}
