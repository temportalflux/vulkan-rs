use crate::utility::{Scissor, Viewport, VulkanInfo};

/// Information about the portion of the viewport a [`Pipeline`](crate::pipeline::Pipeline) should render to.
pub struct ViewportState {
	viewports: Vec<erupt::vk::Viewport>,
	scissors: Vec<erupt::vk::Rect2D>,
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
	pub fn add_viewport(mut self, viewport: Viewport) -> Self {
		self.viewports.push(viewport.into());
		self
	}

	pub fn add_scissor(mut self, scissor: Scissor) -> Self {
		self.scissors.push(scissor.into());
		self
	}
}

impl VulkanInfo<erupt::vk::PipelineViewportStateCreateInfo> for ViewportState {
	fn to_vk(&self) -> erupt::vk::PipelineViewportStateCreateInfo {
		let mut info = erupt::vk::PipelineViewportStateCreateInfo::default();
		info.viewport_count = self.viewports.len() as u32;
		info.p_viewports = self.viewports.as_ptr() as _;
		info.scissor_count = self.scissors.len() as u32;
		info.p_scissors = self.scissors.as_ptr() as _;
		info
	}
}
