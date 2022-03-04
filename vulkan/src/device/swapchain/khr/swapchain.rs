use super::{
	super::{AcquiredImage, ImageAcquisitionBarrier, Swapchain as SwapchainTrait},
	Builder,
};
use crate::{
	backend,
	device::logical,
	flags,
	image::Image,
	image_view::View,
	structs,
	utility::{self, NameableBuilder, NamedObject},
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
}

impl SwapchainTrait for Swapchain {
	fn get_image_views(&self) -> anyhow::Result<Vec<View>> {
		use utility::{BuildFromDevice, HandledObject};

		let mut views = Vec::new();

		let images = unsafe {
			self.device
				.unwrap_swapchain()
				.get_swapchain_images(self.internal)
		}?;
		let images = images
			.into_iter()
			.enumerate()
			// no device reference is passed in because the images are a part of the swapchain
			.map(|(i, image)| {
				let name = self.frame_name(i).map(|v| format!("{}.Image", v));
				let image = Image::from_swapchain(
					image,
					name.clone(),
					self.image_format,
					self.image_extent,
				);
				if let Some(name) = name {
					self.device.set_object_name_logged(&image.create_name(name));
				}
				image
			});

		for image in images {
			views.push(
				View::builder()
					.with_optname(image.name().as_ref().map(|name| format!("{}.View", name)))
					.for_image(sync::Arc::new(image))
					.with_view_type(flags::ImageViewType::TYPE_2D)
					.with_range(
						structs::subresource::Range::default()
							.with_aspect(flags::ImageAspect::COLOR),
					)
					.build(&self.device)?,
			);
		}

		Ok(views)
	}

	/// Determines the index of the image to render the next frame to.
	fn acquire_next_image(
		&self,
		timeout: u64,
		barrier: ImageAcquisitionBarrier,
	) -> anyhow::Result<AcquiredImage> {
		let (semaphore, fence) = match barrier {
			ImageAcquisitionBarrier::Semaphore(semaphore) => {
				(**semaphore, backend::vk::Fence::null())
			}
			ImageAcquisitionBarrier::Fence(fence) => (backend::vk::Semaphore::null(), **fence),
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
