use crate::{backend, command, device::swapchain::khr};

/// Data used to present frames via a [`Queue`](crate::device::logical::Queue).
///
/// It is NOT safe to keep this struct around for more than 1 stack,
/// as it stores unsafe Vulkan handles/pointers.
pub struct PresentInfo {
	semaphores_to_wait_for: Vec<backend::vk::Semaphore>,
	swapchains: Vec<backend::vk::SwapchainKHR>,
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
	/// Adds a signal on the GPU that the command buffer should wait
	/// for before executing the commands in the buffer being presented.
	pub fn wait_for(mut self, semaphore: &command::Semaphore) -> Self {
		self.semaphores_to_wait_for.push(**semaphore);
		self
	}

	/// The swapchain used to present the command's framebuffer.
	pub fn add_swapchain(mut self, swapchain: &khr::Swapchain) -> Self {
		self.swapchains.push(**swapchain);
		self
	}

	/// The index of the image in the swapchain that will be written to.
	pub fn add_image_index(mut self, img: u32) -> Self {
		self.image_indices.push(img);
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::PresentInfoKHR {
		backend::vk::PresentInfoKHR::builder()
			.wait_semaphores(&self.semaphores_to_wait_for)
			.swapchains(&self.swapchains)
			.image_indices(&self.image_indices)
			.build()
	}
}
