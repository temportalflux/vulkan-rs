use super::Buffer;
use crate::{backend, device::logical, image_view, renderpass, structs::Extent2D, utility};
use std::sync;

/// Information used to construct a [`Buffer`].
pub struct SingleBuilder {
	extent: Extent2D,
	layer_count: u32,
	name: String,
}

impl Default for SingleBuilder {
	fn default() -> Self {
		Self {
			extent: Extent2D::default(),
			layer_count: 1,
			name: String::new(),
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
		use utility::HandledObject;
		let attachments = attachments.into_iter().map(|a| **a).collect::<Vec<_>>();
		let info = backend::vk::FramebufferCreateInfo::builder()
			.width(self.extent.width)
			.height(self.extent.height)
			.layers(self.layer_count)
			.render_pass(**render_pass)
			.attachments(&attachments[..])
			.build();
		let vk = unsafe { device.create_framebuffer(&info, None) }?;
		let framebuffer = Buffer::from(device.clone(), vk, self.name.clone());
		device.set_object_name_logged(&framebuffer.create_name(self.name.as_str()));
		Ok(framebuffer)
	}
}

impl utility::NameableBuilder for SingleBuilder {
	fn set_name(&mut self, name: impl Into<String>) {
		self.name = name.into();
	}

	fn name(&self) -> &String {
		&self.name
	}
}
