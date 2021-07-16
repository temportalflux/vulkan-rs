use crate::{
	backend, command,
	device::{logical, swapchain::Builder},
	flags,
	image::Image,
	structs, utility::{self, NamedObject, NameableBuilder},
};

use std::sync;

/// A wrapper struct for [`backend::vk::SwapchainKHR`] to handle swapping out
/// displayed images on the [`Surface`](crate::Surface).
pub struct Swapchain {
	image_format: flags::format::Format,
	image_extent: structs::Extent2D,
	internal: backend::vk::SwapchainKHR,
	device: sync::Arc<logical::Device>,
	name: Option<String>,
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
			name: builder.name().clone(),
		}
	}

	pub fn frame_name(&self, i: usize) -> Option<String> {
		self.name().as_ref().map(|v| format!("{}.Frame{}", v, i))
	}

	/// Creates the swapchain images from the vulkan device.
	pub fn get_images(&self) -> Result<Vec<Image>, utility::Error> {
		use utility::HandledObject;
		Ok(unsafe {
			self.device
				.unwrap_swapchain()
				.get_swapchain_images(self.internal)
		}?
		.into_iter()
		.enumerate()
		// no device reference is passed in because the images are a part of the swapchain
		.map(|(i, image)| {
			let name = self.frame_name(i).map(|v| format!("{}.Image", v));
			let image =
				Image::from_swapchain(image, name.clone(), self.image_format, self.image_extent);
			if let Some(name) = name {
				self.device.set_object_name_logged(&image.create_name(name));
			}
			image
		})
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

impl utility::HandledObject for Swapchain {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::SwapchainKHR as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}

impl NamedObject for Swapchain {
	fn name(&self) -> &Option<String> {
		&self.name
	}
}
