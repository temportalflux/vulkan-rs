use crate::{backend, structs, utility};

pub enum Entries<T> {
	Dynamic(usize),
	Fixed(Vec<T>),
}
impl<T> Default for Entries<T> {
	fn default() -> Self {
		Self::Fixed(Vec::new())
	}
}
impl<T> Entries<T> {
	fn count(&self) -> usize {
		match self {
			Self::Dynamic(count) => *count,
			Self::Fixed(vec) => vec.len(),
		}
	}
}

/// Information about the portion of the viewport a [`Pipeline`](crate::pipeline::Pipeline) should render to.
pub struct Viewport {
	viewports: Entries<backend::vk::Viewport>,
	scissors: Entries<backend::vk::Rect2D>,
}

impl Default for Viewport {
	fn default() -> Self {
		Self {
			viewports: Entries::default(),
			scissors: Entries::default(),
		}
	}
}

impl Viewport {
	pub fn from(resolution: structs::Extent2D) -> Self {
		Self::default()
			.with_viewports(Entries::Fixed(vec![
				utility::Viewport::default().set_size(resolution)
			]))
			.with_scissors(Entries::Fixed(vec![
				utility::Scissor::default().set_size(resolution)
			]))
	}

	pub fn dynamic(viewports: usize, scissors: usize) -> Self {
		Self::default()
			.with_viewports(Entries::Dynamic(viewports))
			.with_scissors(Entries::Dynamic(scissors))
	}

	pub fn with_viewports(mut self, viewports: Entries<utility::Viewport>) -> Self {
		self.viewports = match viewports {
			Entries::Dynamic(count) => Entries::Dynamic(count),
			Entries::Fixed(vec) => Entries::Fixed(vec.into_iter().map(|t| t.into()).collect()),
		};
		self
	}

	pub fn with_scissors(mut self, scissors: Entries<utility::Scissor>) -> Self {
		self.scissors = match scissors {
			Entries::Dynamic(count) => Entries::Dynamic(count),
			Entries::Fixed(vec) => Entries::Fixed(vec.into_iter().map(|t| t.into()).collect()),
		};
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::PipelineViewportStateCreateInfo {
		let mut info = backend::vk::PipelineViewportStateCreateInfo::default();

		info.viewport_count = self.viewports.count() as u32;
		let _vk_viewports: Vec<backend::vk::Viewport>;
		if let Entries::Fixed(vec) = &self.viewports {
			_vk_viewports = vec.clone();
			info.p_viewports = _vk_viewports.as_ptr() as _;
		}

		info.scissor_count = self.scissors.count() as u32;
		let _vk_scissors: Vec<backend::vk::Rect2D>;
		if let Entries::Fixed(vec) = &self.scissors {
			_vk_scissors = vec.clone();
			info.p_scissors = _vk_scissors.as_ptr() as _;
		}

		info
	}
}
