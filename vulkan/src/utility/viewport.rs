use crate::{
	backend,
	structs::{Extent2D, Offset2D},
};

/// A 6-float struct representing the viewport of a window.
pub struct Viewport {
	pos: Offset2D,
	size: Extent2D,
	depth_range: Offset2D,
}

impl Default for Viewport {
	fn default() -> Viewport {
		Viewport {
			pos: Default::default(),
			size: Default::default(),
			depth_range: Offset2D { x: 0, y: 1 },
		}
	}
}

impl Viewport {
	pub fn set_size(mut self, extent: Extent2D) -> Self {
		self.size = extent;
		self
	}
}

impl Into<backend::vk::Viewport> for Viewport {
	fn into(self) -> backend::vk::Viewport {
		backend::vk::Viewport::builder()
			.x(self.pos.x as f32)
			.y(self.pos.y as f32)
			.width(self.size.width as f32)
			.height(self.size.height as f32)
			.min_depth(self.depth_range.x as f32)
			.max_depth(self.depth_range.y as f32)
			.build()
	}
}
