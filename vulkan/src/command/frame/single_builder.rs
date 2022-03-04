use super::Buffer;
use crate::{backend, device::logical, image_view, renderpass, structs::Extent2D, utility};
use std::sync;

/// Information used to construct a [`Buffer`].
pub struct SingleBuilder {
	extent: Extent2D,
	layer_count: u32,
	name: Option<String>,
}

impl Default for SingleBuilder {
	fn default() -> Self {
		Self {
			extent: Extent2D::default(),
			layer_count: 1,
			name: None,
		}
	}
}

impl SingleBuilder {
	pub fn set_extent(mut self, extent: Extent2D) -> Self {
		self.extent = extent;
		self
	}

	pub fn build(
		&self,
		attachments: Vec<&image_view::View>,
		render_pass: &renderpass::Pass,
		device: &sync::Arc<logical::Device>,
	) -> utility::Result<Buffer> {
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
		let framebuffer = Buffer::from(device.clone(), vk);
		if let Some(name) = self.name().as_ref() {
			device.set_object_name_logged(&framebuffer.create_name(name.as_str()));
		}
		Ok(framebuffer)
	}
}

impl utility::NameableBuilder for SingleBuilder {
	fn set_optname(&mut self, name: Option<String>) {
		self.name = name;
	}

	fn name(&self) -> &Option<String> {
		&self.name
	}
}
