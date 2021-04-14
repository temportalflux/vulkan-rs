use erupt;

pub struct Swapchain {
	_internal: erupt::vk::SwapchainKHR,
}

impl Swapchain {
	pub fn new(_internal: erupt::vk::SwapchainKHR) -> Swapchain {
		Swapchain { _internal }
	}
}
