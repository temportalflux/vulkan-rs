use crate::{device::logical, image::Image, utility};
use erupt;
use std::rc::Rc;

/// A wrapper struct for [`erupt::vk::SwapchainKHR`] to handle swapping out
/// displayed images on the [`Surface`](crate::Surface).
pub struct Swapchain {
	_device: Rc<logical::Device>,
	_internal: erupt::vk::SwapchainKHR,
}

impl Swapchain {
	/// The internal constructor. Users should use [`create_object`](crate::device::swapchain::Info::create_object) to create a surface.
	pub fn from(_device: Rc<logical::Device>, _internal: erupt::vk::SwapchainKHR) -> Swapchain {
		Swapchain { _device, _internal }
	}

	pub fn get_images(&self) -> Result<Vec<Image>, utility::Error> {
		Ok(self
			._device
			.get_swapchain_images(&self._internal)?
			.into_iter()
			// no device reference is passed in because the images are a part of the swapchain
			.map(|image| Image::from(None, image))
			.collect())
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::SwapchainKHR`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<erupt::vk::SwapchainKHR> for Swapchain {
	fn unwrap(&self) -> &erupt::vk::SwapchainKHR {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::SwapchainKHR {
		&mut self._internal
	}
}

impl Drop for Swapchain {
	fn drop(&mut self) {
		self._device.destroy_swapchain(self._internal);
	}
}
