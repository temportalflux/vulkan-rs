use crate::{
	backend, structs,
	utility::{Scissor, Viewport},
};

/// Information about the portion of the viewport a [`Pipeline`](crate::pipeline::Pipeline) should render to.
pub struct ViewportState {
	viewports: Vec<backend::vk::Viewport>,
	scissors: Vec<backend::vk::Rect2D>,
}

impl Default for ViewportState {
	fn default() -> ViewportState {
		ViewportState {
			viewports: Vec::new(),
			scissors: Vec::new(),
		}
	}
}

impl ViewportState {
	pub fn from(resolution: structs::Extent2D) -> Self {
		Self::default()
			.add_viewport(Viewport::default().set_size(resolution))
			.add_scissor(Scissor::default().set_size(resolution))
	}

	pub fn add_viewport(mut self, viewport: Viewport) -> Self {
		self.viewports.push(viewport.into());
		self
	}

	pub fn add_scissor(mut self, scissor: Scissor) -> Self {
		self.scissors.push(scissor.into());
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::PipelineViewportStateCreateInfo {
		let mut info = backend::vk::PipelineViewportStateCreateInfo::default();
		info.viewport_count = self.viewports.len() as u32;
		info.p_viewports = self.viewports.as_ptr() as _;
		info.scissor_count = self.scissors.len() as u32;
		info.p_scissors = self.scissors.as_ptr() as _;
		info
	}
}
