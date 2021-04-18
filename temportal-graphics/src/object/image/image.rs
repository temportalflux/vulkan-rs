use crate::{device::logical, utility::VulkanObject};
use erupt;
use std::rc::Rc;

/// An handle representing image data stored on the [`GPU`](crate::device::physical::Device),
/// including any created by the [`Swapchain`](crate::device::swapchain::Swapchain).
pub struct Image {
	_device: Option<Rc<logical::Device>>, // empty for images created from the swapchain
	_internal: erupt::vk::Image,
}

impl Image {
	pub fn from(_device: Option<Rc<logical::Device>>, _internal: erupt::vk::Image) -> Image {
		Image { _device, _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::Image`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::Image> for Image {
	fn unwrap(&self) -> &erupt::vk::Image {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::Image {
		&mut self._internal
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		if let Some(device) = &self._device {
			device.destroy_image(self._internal);
		}
	}
}
