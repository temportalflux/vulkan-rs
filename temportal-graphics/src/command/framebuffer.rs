use crate::{
	backend, device::logical, image, renderpass, structs::Extent2D, utility, utility::VulkanObject,
};
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
		device: &Rc<logical::Device>,
	) -> utility::Result<Framebuffer> {
		use backend::version::DeviceV1_0;
		let attachments = vec![*swapchain_image_view.unwrap()];
		let info = backend::vk::FramebufferCreateInfo::builder()
			.width(self.extent.width)
			.height(self.extent.height)
			.layers(self.layer_count)
			.render_pass(*render_pass.unwrap())
			.attachments(&attachments[..])
			.build();
		let vk =
			utility::as_vulkan_error(unsafe { device.unwrap().create_framebuffer(&info, None) })?;
		Ok(Framebuffer::from(device.clone(), vk))
	}
}

pub struct Framebuffer {
	internal: backend::vk::Framebuffer,
	device: Rc<logical::Device>,
}

impl Framebuffer {
	pub fn from(device: Rc<logical::Device>, internal: backend::vk::Framebuffer) -> Framebuffer {
		Framebuffer { device, internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Framebuffer`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::Framebuffer> for Framebuffer {
	fn unwrap(&self) -> &backend::vk::Framebuffer {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Framebuffer {
		&mut self.internal
	}
}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.unwrap()
				.destroy_framebuffer(self.internal, None)
		};
	}
}
