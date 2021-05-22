use crate::{backend, structs::Extent2D};
use temportal_math::Vector;

/// A 4-int struct representing a portion of a [`Viewport`](crate::utility::Viewport).
#[derive(Debug)]
pub struct Scissor {
	offset: Vector<i32, 2>,
	size: Vector<u32, 2>,
}

impl Default for Scissor {
	fn default() -> Scissor {
		Scissor {
			offset: Vector::filled(0),
			size: Vector::filled(0),
		}
	}
}

impl Scissor {
	pub fn new(offset: Vector<i32, 2>, size: Vector<u32, 2>) -> Self {
		Self { offset, size }
	}

	pub fn set_size(mut self, extent: Extent2D) -> Self {
		*self.size.x_mut() = extent.width;
		*self.size.y_mut() = extent.height;
		self
	}
}

impl Into<backend::vk::Rect2D> for Scissor {
	fn into(self) -> backend::vk::Rect2D {
		backend::vk::Rect2D::builder()
			.offset(backend::vk::Offset2D {
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
