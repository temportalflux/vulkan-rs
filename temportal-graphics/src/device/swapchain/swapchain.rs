use crate::{
	backend, command,
	device::logical,
	image::Image,
	utility::{self, VulkanObject},
};

use std::rc::Rc;

/// A wrapper struct for [`backend::vk::SwapchainKHR`] to handle swapping out
/// displayed images on the [`Surface`](crate::Surface).
pub struct Swapchain {
	device: Rc<logical::Device>,
	internal: backend::vk::SwapchainKHR,
}

impl Swapchain {
	/// The internal constructor. Users should use [`create_object`](crate::device::swapchain::Info::create_object) to create a surface.
	pub fn from(device: Rc<logical::Device>, internal: backend::vk::SwapchainKHR) -> Swapchain {
		Swapchain { device, internal }
	}

	pub fn get_images(&self) -> Result<Vec<Image>, utility::Error> {
		Ok(utility::as_vulkan_error(unsafe {
			self.device
				.unwrap_swapchain()
				.get_swapchain_images(self.internal)
		})?
		.into_iter()
		// no device reference is passed in because the images are a part of the swapchain
		.map(|image| Image::from_swapchain(image))
		.collect())
	}

	/// returns (_, true) if the swapchain is suboptimal
	pub fn acquire_next_image(
		&self,
		timeout: u64,
		semaphore: Option<&command::Semaphore>,
		fence: Option<&command::Fence>,
	) -> utility::Result<(/*image index*/ u32, /*is suboptimal*/ bool)> {
		utility::as_vulkan_error(unsafe {
			self.device.unwrap_swapchain().acquire_next_image(
				self.internal,
				timeout,
				semaphore.map_or(backend::vk::Semaphore::null(), |obj| *obj.unwrap()),
				fence.map_or(backend::vk::Fence::null(), |obj| *obj.unwrap()),
			)
		})
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::SwapchainKHR`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::SwapchainKHR> for Swapchain {
	fn unwrap(&self) -> &backend::vk::SwapchainKHR {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::SwapchainKHR {
		&mut self.internal
	}
}

impl Drop for Swapchain {
	fn drop(&mut self) {
		unsafe {
			self.device
				.unwrap_swapchain()
				.destroy_swapchain(self.internal, None)
		};
	}
}
