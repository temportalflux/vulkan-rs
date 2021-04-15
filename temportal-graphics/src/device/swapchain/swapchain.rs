use crate::{device::logical, image::Image, utility::VulkanObject};
use erupt;

pub struct Swapchain {
	_internal: erupt::vk::SwapchainKHR,
}

impl Swapchain {
	pub fn new(_internal: erupt::vk::SwapchainKHR) -> Swapchain {
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
