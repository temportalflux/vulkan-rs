use crate::{backend, command, device::logical, image::Image, utility};

use std::rc::Rc;

/// A wrapper struct for [`backend::vk::SwapchainKHR`] to handle swapping out
/// displayed images on the [`Surface`](crate::Surface).
pub struct Swapchain {
	_device: Rc<logical::Device>,
	_internal: backend::vk::SwapchainKHR,
}

impl Swapchain {
	/// The internal constructor. Users should use [`create_object`](crate::device::swapchain::Info::create_object) to create a surface.
	pub fn from(_device: Rc<logical::Device>, _internal: backend::vk::SwapchainKHR) -> Swapchain {
		Swapchain { _device, _internal }
	}

	pub fn get_images(&self) -> Result<Vec<Image>, utility::Error> {
		Ok(self
			._device
			.get_swapchain_images(&self._internal)?
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
	) -> utility::Result<(u32, bool)> {
		self._device
			.acquire_next_image(&self, timeout, semaphore, fence)
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::SwapchainKHR`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<backend::vk::SwapchainKHR> for Swapchain {
	fn unwrap(&self) -> &backend::vk::SwapchainKHR {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::SwapchainKHR {
		&mut self._internal
	}
}

impl Drop for Swapchain {
	fn drop(&mut self) {
		self._device.destroy_swapchain(self._internal);
	}
}
