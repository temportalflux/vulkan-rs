use crate::{backend, device::logical, utility};
use std::sync;

/// Buffers represent a collection of specific memory attachments that a render pass instance uses.
/// This is something that needs to exist for rendering frames, but that has no data that you can use or access.
///
/// Equivalent to [`VkFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebuffer.html).
pub struct Buffer {
	internal: backend::vk::Framebuffer,
	device: sync::Arc<logical::Device>,
}

impl Buffer {
	pub fn builder() -> super::SingleBuilder {
		super::SingleBuilder::default()
	}

	pub fn multi_builder() -> super::MultiBuilder {
		super::MultiBuilder::default()
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::Framebuffer,
	) -> Self {
		Self { device, internal }
	}
}

impl std::ops::Deref for Buffer {
	type Target = backend::vk::Framebuffer;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		unsafe { self.device.destroy_framebuffer(self.internal, None) };
	}
}

impl utility::HandledObject for Buffer {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Framebuffer as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
