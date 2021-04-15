use crate::structs::Extent2D;
use temportal_math::Vector;

pub struct Viewport {
	pos: Vector<f32, 2>,
	size: Vector<f32, 2>,
	depth_range: Vector<f32, 2>,
}

impl Viewport {
	pub fn new() -> Viewport {
		Viewport {
			pos: Vector::filled(0.0),
			size: Vector::filled(0.0),
			depth_range: Vector::new([0.0, 1.0]),
		}
	}

	pub fn set_size(mut self, extent: Extent2D) -> Self {
		*self.size.x_mut() = extent.width as f32;
		*self.size.y_mut() = extent.height as f32;
		self
	}
}

impl Into<erupt::vk::Viewport> for Viewport {
	fn into(self) -> erupt::vk::Viewport {
		erupt::vk::ViewportBuilder::new()
			.x(self.pos.x())
			.y(self.pos.y())
			.width(self.size.x())
			.height(self.size.y())
			.min_depth(self.depth_range.x())
			.max_depth(self.depth_range.y())
			.build()
	}
}
