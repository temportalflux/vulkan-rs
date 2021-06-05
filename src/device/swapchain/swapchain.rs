use crate::{
	backend, command,
	device::{logical, swapchain::Builder},
	flags,
	image::Image,
	structs, utility,
};

use std::sync;

/// A wrapper struct for [`backend::vk::SwapchainKHR`] to handle swapping out
/// displayed images on the [`Surface`](crate::Surface).
pub struct Swapchain {
	image_format: flags::format::Format,
	image_extent: structs::Extent2D,
	internal: backend::vk::SwapchainKHR,
	device: sync::Arc<logical::Device>,
}

impl Swapchain {
	/// Helper method for creating a default swapchain builder.
	pub fn builder() -> Builder {
		Builder::default()
	}

	/// Constructs the swapchain object from a completed [`Builder`].
	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::SwapchainKHR,
		builder: Builder,
	) -> Swapchain {
		Swapchain {
			device,
			internal,
			image_format: builder.image_format,
			image_extent: builder.image_extent,
		}
	}

	/// Creates the swapchain images from the vulkan device.
	pub fn get_images(&self) -> Result<Vec<Image>, utility::Error> {
		Ok(unsafe {
			self.device
				.unwrap_swapchain()
				.get_swapchain_images(self.internal)
		}?
		.into_iter()
		// no device reference is passed in because the images are a part of the swapchain
		.map(|image| Image::from_swapchain(image, self.image_format, self.image_extent))
		.collect())
	}

	/// Determines the image index of [`get_images`](Swapchain::get_images)
	/// to render the next frame to.
	/// Returns `(<index>, true)` if the swapchain is suboptimal.
	pub fn acquire_next_image(
		&self,
		timeout: u64,
		semaphore: Option<&command::Semaphore>,
		fence: Option<&command::Fence>,
	) -> utility::Result<(/*image index*/ u32, /*is suboptimal*/ bool)> {
		Ok(unsafe {
			self.device.unwrap_swapchain().acquire_next_image(
				self.internal,
				timeout,
				semaphore.map_or(backend::vk::Semaphore::null(), |obj| **obj),
				fence.map_or(backend::vk::Fence::null(), |obj| **obj),
			)
		}?)
	}
}

impl std::ops::Deref for Swapchain {
	type Target = backend::vk::SwapchainKHR;
	fn deref(&self) -> &Self::Target {
		&self.internal
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