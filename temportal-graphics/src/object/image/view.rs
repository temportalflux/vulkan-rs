use crate::{backend, device::logical, utility::VulkanObject};

use std::rc::Rc;

/// A wrapper around [`Image View`](backend::vk::ImageView).
pub struct View {
	internal: backend::vk::ImageView,
	device: Rc<logical::Device>,
}

impl View {
	pub fn from(device: Rc<logical::Device>, internal: backend::vk::ImageView) -> View {
		View { device, internal }
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
