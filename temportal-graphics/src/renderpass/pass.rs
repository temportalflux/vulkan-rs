use crate::utility::VulkanObject;
use erupt;

/// The [`Render Pass`](erupt::vk::RenderPass) used by Vulkan
/// to define when pipeline instructions can be issued
/// and what attachments are used.
pub struct Pass {
	_internal: erupt::vk::RenderPass,
}

impl Pass {
	pub fn from(_internal: erupt::vk::RenderPass) -> Pass {
		Pass { _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::RenderPass`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::RenderPass> for Pass {
	fn unwrap(&self) -> &erupt::vk::RenderPass {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::RenderPass {
		&mut self._internal
	}
}
