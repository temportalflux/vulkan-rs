use crate::{
	command,
	device::swapchain::Swapchain,
	utility::{VulkanInfo, VulkanObject},
};
use erupt;

/// Data used to present frames via a [`Queue`](crate::device::logical::Queue).
/// It is NOT safe to keep this struct around for more than 1 stack,
/// as it stores unsafe Vulkan handles/pointers.
pub struct PresentInfo {
	semaphores_to_wait_for: Vec<erupt::vk::Semaphore>,
	swapchains: Vec<erupt::vk::SwapchainKHR>,
	image_indices: Vec<u32>,
}

impl Default for PresentInfo {
	fn default() -> PresentInfo {
		PresentInfo {
			semaphores_to_wait_for: Vec::new(),
			swapchains: Vec::new(),
			image_indices: Vec::new(),
		}
	}
}

impl PresentInfo {
	pub fn wait_for(mut self, semaphore: &command::Semaphore) -> Self {
		self.semaphores_to_wait_for.push(*semaphore.unwrap());
		self
	}

	pub fn add_swapchain(mut self, swapchain: &Swapchain) -> Self {
		self.swapchains.push(*swapchain.unwrap());
		self
	}

	pub fn add_image_index(mut self, img: u32) -> Self {
		self.image_indices.push(img);
		self
	}
}

impl VulkanInfo<erupt::vk::PresentInfoKHR> for PresentInfo {
	fn to_vk(&self) -> erupt::vk::PresentInfoKHR {
		erupt::vk::PresentInfoKHRBuilder::new()
			.wait_semaphores(&self.semaphores_to_wait_for)
			.swapchains(&self.swapchains)
			.image_indices(&self.image_indices)
			.build()
	}
}
