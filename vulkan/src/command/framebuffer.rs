use crate::{backend, device::logical, image_view, renderpass, structs::Extent2D, utility};
use std::sync;

/// Information used to construct a [`Framebuffer`].
pub struct Builder {
	extent: Extent2D,
	layer_count: u32,
}

impl Default for Builder {
	fn default() -> Self {
		Self {
			extent: Extent2D::default(),
			layer_count: 1,
		}
	}
}

impl Builder {
	pub fn set_extent(mut self, extent: Extent2D) -> Self {
		self.extent = extent;
		self
	}

	pub fn build(
		&self,
		swapchain_image_view: &image_view::View,
		render_pass: &renderpass::Pass,
		device: &sync::Arc<logical::Device>,
	) -> utility::Result<Framebuffer> {
		use backend::version::DeviceV1_0;
		let attachments = vec![**swapchain_image_view];
		let info = backend::vk::FramebufferCreateInfo::builder()
			.width(self.extent.width)
			.height(self.extent.height)
			.layers(self.layer_count)
			.render_pass(**render_pass)
			.attachments(&attachments[..])
			.build();
		let vk = unsafe { device.create_framebuffer(&info, None) }?;
		Ok(Framebuffer::from(device.clone(), vk))
	}
}

/// Framebuffers represent a collection of specific memory attachments that a render pass instance uses.
/// This is something that needs to exist for rendering frames, but that has no data that you can use or access.
///
/// Equivalent to [`VkFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebuffer.html).
pub struct Framebuffer {
	internal: backend::vk::Framebuffer,
	device: sync::Arc<logical::Device>,
}

impl Framebuffer {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::Framebuffer,
	) -> Framebuffer {
		Framebuffer { device, internal }
	}
}

impl std::ops::Deref for Framebuffer {
	type Target = backend::vk::Framebuffer;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.destroy_framebuffer(self.internal, None) };
	}
}
