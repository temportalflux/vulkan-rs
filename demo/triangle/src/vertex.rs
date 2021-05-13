use crate::engine::{
	graphics::{self, flags, pipeline},
	math::Vector,
};

pub struct Vertex {
	pub pos: Vector<f32, 2>,
	_pos_padding: [f32; 2],
	pub color: Vector<f32, 4>,
}

impl Default for Vertex {
	fn default() -> Vertex {
		Vertex {
			pos: Vector::default(),
			_pos_padding: [0.0, 0.0],
			color: Vector::default(),
		}
	}
}

impl Vertex {
	pub fn with_pos(mut self, pos: Vector<f32, 2>) -> Self {
		self.pos = pos;
		self
	}
	pub fn with_color(mut self, color: Vector<f32, 4>) -> Self {
		self.color = color;
		self
	}
}

impl pipeline::vertex::Object for Vertex {
	fn attributes() -> Vec<pipeline::vertex::Attribute> {
		vec![
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Vertex, pos),
				format: flags::Format::R32G32_SFLOAT,
			},
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Vertex, color),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
		]
	}
}
