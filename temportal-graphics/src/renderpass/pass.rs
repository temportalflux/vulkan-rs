use crate::{device::logical, utility::VulkanObject};
use erupt;
use std::rc::Rc;

/// The [`Render Pass`](erupt::vk::RenderPass) used by Vulkan
/// to define when pipeline instructions can be issued
/// and what attachments are used.
pub struct Pass {
	_device: Rc<logical::Device>,
	_internal: erupt::vk::RenderPass,
}

impl Pass {
	pub fn from(_device: Rc<logical::Device>, _internal: erupt::vk::RenderPass) -> Pass {
		Pass { _device, _internal }
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

impl Drop for Pass {
	fn drop(&mut self) {
		self._device.destroy_render_pass(self._internal)
	}
}
