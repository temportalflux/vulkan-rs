use crate::{backend, device::logical, utility::VulkanObject};

use std::sync;

/// The [`Render Pass`](backend::vk::RenderPass) used by Vulkan
/// to define when pipeline instructions can be issued
/// and what attachments are used.
pub struct Pass {
	internal: backend::vk::RenderPass,
	device: sync::Arc<logical::Device>,
}

impl Pass {
	pub fn from(device: sync::Arc<logical::Device>, internal: backend::vk::RenderPass) -> Pass {
		Pass { device, internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::RenderPass`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::RenderPass> for Pass {
	fn unwrap(&self) -> &backend::vk::RenderPass {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::RenderPass {
		&mut self.internal
	}
}

impl Drop for Pass {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.unwrap()
				.destroy_render_pass(self.internal, None)
		};
	}
}
