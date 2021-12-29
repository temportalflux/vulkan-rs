use crate::{backend, device::logical, image_view, renderpass, structs::Extent2D, utility};
use std::sync;

/// Information used to construct a [`Framebuffer`].
pub struct Builder {
	extent: Extent2D,
	layer_count: u32,
	name: Option<String>,
}

impl Default for Builder {
	fn default() -> Self {
		Self {
			extent: Extent2D::default(),
			layer_count: 1,
			name: None,
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
		attachments: Vec<&image_view::View>,
		render_pass: &renderpass::Pass,
		device: &sync::Arc<logical::Device>,
	) -> utility::Result<Framebuffer> {
		use backend::version::DeviceV1_0;
		use utility::{HandledObject, NameableBuilder};
		let attachments = attachments.into_iter().map(|a| **a).collect::<Vec<_>>();
		let info = backend::vk::FramebufferCreateInfo::builder()
			.width(self.extent.width)
			.height(self.extent.height)
			.layers(self.layer_count)
			.render_pass(**render_pass)
			.attachments(&attachments[..])
			.build();
		let vk = unsafe { device.create_framebuffer(&info, None) }?;
		let framebuffer = Framebuffer::from(device.clone(), vk);
		if let Some(name) = self.name().as_ref() {
			device.set_object_name_logged(&framebuffer.create_name(name.as_str()));
		}
		Ok(framebuffer)
	}
}

impl utility::NameableBuilder for Builder {
	fn set_optname(&mut self, name: Option<String>) {
		self.name = name;
	}

	fn name(&self) -> &Option<String> {
		&self.name
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

impl utility::HandledObject for Framebuffer {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Framebuffer as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
