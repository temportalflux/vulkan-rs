use crate::{
	device::logical, image, renderpass, structs::Extent2D, utility, utility::VulkanObject,
};
use erupt;

/// Information used to construct a [`Framebuffer`].
pub struct Info {
	extent: Extent2D,
	layer_count: u32,
}

impl Default for Info {
	fn default() -> Info {
		Info {
			extent: Extent2D::default(),
			layer_count: 1,
		}
	}
}

impl Info {
	pub fn set_extent(mut self, extent: Extent2D) -> Self {
		self.extent = extent;
		self
	}

	pub fn create_object(
		&self,
		swapchain_image_view: &image::View,
		render_pass: &renderpass::Pass,
		device: &logical::Device,
	) -> utility::Result<Framebuffer> {
		let attachments = vec![*swapchain_image_view.unwrap()];
		let info = erupt::vk::FramebufferCreateInfoBuilder::new()
			.width(self.extent.width)
			.height(self.extent.height)
			.layers(self.layer_count)
			.render_pass(*render_pass.unwrap())
			.attachments(&attachments[..])
			.build();
		Ok(Framebuffer::from(device.create_framebuffer(info)?))
	}
}

pub struct Framebuffer {
	_internal: erupt::vk::Framebuffer,
}

impl Framebuffer {
	pub fn from(_internal: erupt::vk::Framebuffer) -> Framebuffer {
		Framebuffer { _internal }
	}
}
