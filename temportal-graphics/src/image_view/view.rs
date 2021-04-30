use crate::{backend, device::logical, image, image_view::Builder, utility::VulkanObject};

use std::rc::Rc;

/// A wrapper around [`Image View`](backend::vk::ImageView).
pub struct View {
	internal: backend::vk::ImageView,
	image: Rc<image::Image>,
	device: Rc<logical::Device>,
}

impl View {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub fn from(
		device: Rc<logical::Device>,
		image: Rc<image::Image>,
		internal: backend::vk::ImageView,
	) -> View {
		View {
			device,
			image,
			internal,
		}
	}

	pub fn image(&self) -> &Rc<image::Image> {
		&self.image
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::ImageView`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::ImageView> for View {
	fn unwrap(&self) -> &backend::vk::ImageView {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::ImageView {
		&mut self.internal
	}
}

impl Drop for View {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.unwrap().destroy_image_view(self.internal, None) };
	}
}
