use crate::utility::VulkanObject;
use erupt;

/// An handle representing image data stored on the [`GPU`](crate::device::physical::Device),
/// including any created by the [`Swapchain`](crate::device::swapchain::Swapchain).
pub struct Image {
	_internal: erupt::vk::Image,
}

impl Image {
	pub fn from(_internal: erupt::vk::Image) -> Image {
		Image { _internal }
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
