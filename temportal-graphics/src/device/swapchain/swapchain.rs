use crate::{device::logical, image::Image, utility::VulkanObject};
use erupt;

/// A wrapper struct for [`erupt::vk::SwapchainKHR`] to handle swapping out
/// displayed images on the [`Surface`](crate::Surface).
pub struct Swapchain {
	_internal: erupt::vk::SwapchainKHR,
}

impl Swapchain {

	/// The internal constructor. Users should use [`create_object`](crate::device::swapchain::Info::create_object) to create a surface.
	pub fn from(_internal: erupt::vk::SwapchainKHR) -> Swapchain {
		Swapchain { _internal }
	}

	pub fn get_images(&self, device: &logical::Device) -> Vec<Image> {
		device
			.get_swapchain_images(&self._internal)
			.into_iter()
			.map(|image| Image::from(image))
			.collect()
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::SwapchainKHR`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::SwapchainKHR> for Swapchain {
	fn unwrap(&self) -> &erupt::vk::SwapchainKHR {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::SwapchainKHR {
		&mut self._internal
	}
}
