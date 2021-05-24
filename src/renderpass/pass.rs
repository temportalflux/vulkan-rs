use crate::{backend, device::logical};

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

impl std::ops::Deref for Pass {
	type Target = backend::vk::RenderPass;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Pass {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.destroy_render_pass(self.internal, None) };
	}
}
