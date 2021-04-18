use crate::{
	device::logical, image, renderpass, structs::Extent2D, utility, utility::VulkanObject,
};
use erupt;
use std::rc::Rc;

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
		device: Rc<logical::Device>,
	) -> utility::Result<Framebuffer> {
		let attachments = vec![*swapchain_image_view.unwrap()];
		let info = erupt::vk::FramebufferCreateInfoBuilder::new()
			.width(self.extent.width)
			.height(self.extent.height)
			.layers(self.layer_count)
			.render_pass(*render_pass.unwrap())
			.attachments(&attachments[..])
			.build();
		let vk = device.create_framebuffer(info)?;
		Ok(Framebuffer::from(device, vk))
	}
}

pub struct Framebuffer {
	_device: Rc<logical::Device>,
	_internal: erupt::vk::Framebuffer,
}

impl Framebuffer {
	pub fn from(_device: Rc<logical::Device>, _internal: erupt::vk::Framebuffer) -> Framebuffer {
		Framebuffer { _device, _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::Framebuffer`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::Framebuffer> for Framebuffer {
	fn unwrap(&self) -> &erupt::vk::Framebuffer {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::Framebuffer {
		&mut self._internal
	}
}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		self._device.destroy_framebuffer(self._internal)
	}
}