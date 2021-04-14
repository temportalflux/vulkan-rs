use crate::{device::logical, Image};
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
