use crate::engine::{
	graphics::{self, flags, pipeline},
	math::*,
};

#[derive(Debug, Clone)]
pub struct Instance {
	pub model: Matrix<f32, 4, 4>,
	pub color: Vector<f32, 4>,
}

impl Default for Instance {
	fn default() -> Instance {
		Instance {
			model: Matrix::identity(),
			color: Vector::default(),
		}
	}
}

impl Instance {
	pub fn with_pos(mut self, pos: Vector<f32, 3>) -> Self {
		self.model *= Matrix::translate(pos);
		self
	}

	pub fn with_orientation(mut self, orientation: Quaternion) -> Self {
		self.model *= orientation.into();
		self
	}

	pub fn with_color(mut self, color: Vector<f32, 4>) -> Self {
		self.color = color;
		self
	}
}

impl pipeline::vertex::Object for Instance {
	fn attributes() -> Vec<pipeline::vertex::Attribute> {
		let matrix_offset = graphics::utility::offset_of!(Instance, model);
		vec![
			pipeline::vertex::Attribute {
				offset: matrix_offset + (std::mem::size_of::<Vector<f32, 4>>() * 0),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
			pipeline::vertex::Attribute {
				offset: matrix_offset + (std::mem::size_of::<Vector<f32, 4>>() * 1),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
			pipeline::vertex::Attribute {
				offset: matrix_offset + (std::mem::size_of::<Vector<f32, 4>>() * 2),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
			pipeline::vertex::Attribute {
				offset: matrix_offset + (std::mem::size_of::<Vector<f32, 4>>() * 3),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Instance, color),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
		]
	}
}
