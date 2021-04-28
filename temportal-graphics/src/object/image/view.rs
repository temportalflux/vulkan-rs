use crate::{backend, device::logical, utility::VulkanObject};

use std::rc::Rc;

/// A wrapper around [`Image View`](backend::vk::ImageView).
pub struct View {
	_device: Rc<logical::Device>,
	_internal: backend::vk::ImageView,
}

impl View {
	pub fn from(_device: Rc<logical::Device>, _internal: backend::vk::ImageView) -> View {
		View { _device, _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::ImageView`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::ImageView> for View {
	fn unwrap(&self) -> &backend::vk::ImageView {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::ImageView {
		&mut self._internal
	}
}

impl Drop for View {
	fn drop(&mut self) {
		self._device.destroy_image_view(self._internal);
	}
}
