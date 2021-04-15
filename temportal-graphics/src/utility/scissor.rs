use crate::structs::Extent2D;
use temportal_math::Vector;

pub struct Scissor {
	offset: Vector<i32, 2>,
	size: Vector<u32, 2>,
}

impl Scissor {
	pub fn new() -> Scissor {
		Scissor {
			offset: Vector::filled(0),
			size: Vector::filled(0),
		}
	}

	pub fn set_size(mut self, extent: Extent2D) -> Self {
		*self.size.x_mut() = extent.width;
		*self.size.y_mut() = extent.height;
		self
	}
}

impl Into<erupt::vk::Rect2D> for Scissor {
	fn into(self) -> erupt::vk::Rect2D {
		erupt::vk::Rect2DBuilder::new()
			.offset(erupt::vk::Offset2D {
				x: self.offset.x(),
				y: self.offset.y(),
			})
			.extent(Extent2D {
				width: self.size.x(),
				height: self.size.x(),
			})
			.build()
	}
}
