use crate::{
	backend,
	structs::{Extent2D, Offset2D},
};

/// A 4-int struct representing a portion of a [`Viewport`](crate::utility::Viewport).
#[derive(Debug, Clone, Copy)]
pub struct Scissor {
	offset: Offset2D,
	size: Extent2D,
}

impl Default for Scissor {
	fn default() -> Scissor {
		Scissor {
			offset: Default::default(),
			size: Default::default(),
		}
	}
}

impl std::fmt::Display for Scissor {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"<{}, {}, {}, {}>",
			self.offset.x, self.offset.y, self.size.width, self.size.height
		)
	}
}

impl Scissor {
	pub fn new(offset: Offset2D, size: Extent2D) -> Self {
		Self { offset, size }
	}

	pub fn set_size(mut self, extent: Extent2D) -> Self {
		self.size = extent;
		self
	}
}

impl Into<backend::vk::Rect2D> for Scissor {
	fn into(self) -> backend::vk::Rect2D {
		backend::vk::Rect2D::builder()
			.offset(self.offset)
			.extent(self.size)
			.build()
	}
}
