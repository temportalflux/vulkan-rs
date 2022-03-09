use super::{
	super::{AcquiredImage, Swapchain as SwapchainTrait},
	Builder,
};
use crate::{
	backend,
	command::{self, SyncObject},
	device::logical,
	flags,
	image::Image,
	structs,
	utility::{self, NameableBuilder, NamedObject},
};
use std::sync::{self, Arc};

/// A wrapper struct for [`backend::vk::SwapchainKHR`] to handle swapping out
/// displayed images on the [`Surface`](crate::Surface).
pub struct Swapchain {
	image_format: flags::format::Format,
	image_extent: structs::Extent2D,
	internal: backend::vk::SwapchainKHR,
	device: sync::Arc<logical::Device>,
	name: Option<String>,
	image_count: usize,
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
		use super::super::SwapchainBuilder;
		Swapchain {
			device,
			internal,
			image_format: builder.image_format,
			image_extent: builder.image_extent,
			name: builder.name().clone(),
			image_count: builder.image_count() as usize,
		}
	}

	pub fn frame_name(&self, i: usize) -> Option<String> {
		self.name().as_ref().map(|v| format!("{}.Frame{}", v, i))
	}
}

impl SwapchainTrait for Swapchain {
	fn device(&self) -> &Arc<logical::Device> {
		&self.device
	}

	fn as_khr(&self) -> Option<&Self> {
		Some(self)
	}

	fn image_count(&self) -> usize {
		self.image_count
	}

	fn image_extent(&self) -> &structs::Extent2D {
		&self.image_extent
	}

	fn create_images(&self) -> anyhow::Result<Vec<Arc<Image>>> {
		use utility::HandledObject;
		let images = unsafe {
			self.device
				.unwrap_swapchain()
				.get_swapchain_images(self.internal)
		}?;
		let images = images.into_iter().enumerate();
		// no device reference is passed in because the images are a part of the swapchain
		let images = images.map(|(i, image)| {
			let name = self.frame_name(i).map(|v| format!("{}.Image", v));
			let image =
				Image::from_swapchain(image, name.clone(), self.image_format, self.image_extent);
			if let Some(name) = name {
				self.device.set_object_name_logged(&image.create_name(name));
			}
			Arc::new(image)
		});
		Ok(images.collect())
	}

	/// Determines the index of the image to render the next frame to.
	fn acquire_next_image(
		&self,
		timeout: u64,
		barrier: SyncObject,
	) -> anyhow::Result<AcquiredImage> {
		let (semaphore, fence) = match barrier {
			SyncObject::Semaphore(semaphore) => (**semaphore, backend::vk::Fence::null()),
			SyncObject::Fence(fence) => (backend::vk::Semaphore::null(), **fence),
		};
		let (index, is_suboptimal) = unsafe {
			self.device.unwrap_swapchain().acquire_next_image(
				self.internal,
				timeout,
				semaphore,
				fence,
			)
		}?;
		Ok(match is_suboptimal {
			true => AcquiredImage::Suboptimal(index as usize),
			false => AcquiredImage::Available(index as usize),
		})
	}
}

impl Swapchain {
	/// Presents the display swapchain to the window.
	pub fn present(
		&self,
		graphics_queue: &Arc<logical::Queue>,
		present_info: command::PresentInfo,
	) -> utility::Result<bool> {
		graphics_queue.present(present_info.add_swapchain(&self))
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
