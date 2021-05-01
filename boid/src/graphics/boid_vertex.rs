use crate::engine::{
	graphics::{self, flags, pipeline},
	math::Vector,
};

pub struct Vertex {
	pub pos: Vector<f32, 2>,
	_pos_padding: [f32; 2],
	pub tex_coord: Vector<f32, 2>,
	_tc_padding: [f32; 2],
}

impl Default for Vertex {
	fn default() -> Vertex {
		Vertex {
			pos: Vector::default(),
			tex_coord: Vector::default(),
			_pos_padding: [0.0, 0.0],
			_tc_padding: [0.0, 0.0],
		}
	}
}

impl Vertex {
	pub fn with_pos(mut self, pos: Vector<f32, 2>) -> Self {
		self.pos = pos;
		self
	}
	pub fn with_tex_coord(mut self, texture_coordinate: Vector<f32, 2>) -> Self {
		self.tex_coord = texture_coordinate;
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
				offset: graphics::utility::offset_of!(Vertex, tex_coord),
				format: flags::Format::R32G32_SFLOAT,
			},
		]
	}
}
