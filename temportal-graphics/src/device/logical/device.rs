use crate::utility::VulkanObject;
use erupt;

pub struct Device {
	_internal: erupt::DeviceLoader,
}

impl Device {
	pub fn new(internal: erupt::DeviceLoader) -> Device {
		Device {
			_internal: internal,
		}
	}
}

impl VulkanObject<erupt::DeviceLoader> for Device {
	fn unwrap(&self) -> &erupt::DeviceLoader {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::DeviceLoader {
		&mut self._internal
	}
}

impl Device {
	pub fn create_swapchain(
		&self,
		info: erupt::vk::SwapchainCreateInfoKHR,
	) -> erupt::vk::SwapchainKHR {
		unsafe { self._internal.create_swapchain_khr(&info, None, None) }.unwrap()
	}

	pub fn get_swapchain_images(
		&self,
		swapchain: &erupt::vk::SwapchainKHR,
	) -> Vec<erupt::vk::Image> {
		unsafe { self._internal.get_swapchain_images_khr(*swapchain, None) }.unwrap()
	}
}
