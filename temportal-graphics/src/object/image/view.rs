use crate::{device::logical, utility::VulkanObject};
use erupt;
use std::rc::Rc;

/// A wrapper around [`Image View`](erupt::vk::ImageView).
pub struct View {
	_device: Rc<logical::Device>,
	_internal: erupt::vk::ImageView,
}

impl View {
	pub fn from(_device: Rc<logical::Device>, _internal: erupt::vk::ImageView) -> View {
		View { _device, _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::ImageView`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::ImageView> for View {
	fn unwrap(&self) -> &erupt::vk::ImageView {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::ImageView {
		&mut self._internal
	}
}

impl Drop for View {
	fn drop(&mut self) {
		self._device.destroy_image_view(self._internal);
	}
}
