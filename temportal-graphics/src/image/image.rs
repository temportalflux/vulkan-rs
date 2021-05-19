use crate::{backend, flags, image, utility};
use std::sync;
use temportal_math::Vector;

pub trait Owner: Send + Sync {
	fn destroy(&self, obj: &Image, allocation: Option<&vk_mem::Allocation>) -> utility::Result<()>;
}

/// An handle representing image data stored on the [`GPU`](crate::device::physical::Device),
/// including any created by the [`Swapchain`](crate::device::swapchain::Swapchain).
pub struct Image {
	image_info: Option<image::Builder>,
	allocation_info: Option<vk_mem::AllocationInfo>,
	allocation_handle: Option<vk_mem::Allocation>,
	internal: backend::vk::Image,
	owner: Option<sync::Arc<dyn Owner>>, // empty for images created from the swapchain
}

impl Image {
	pub fn from_swapchain(internal: backend::vk::Image) -> Image {
		Image {
			owner: None,
			internal,
			allocation_handle: None,
			allocation_info: None,
			image_info: None,
		}
	}

	pub fn builder() -> image::Builder {
		image::Builder::default()
	}

	pub fn new(
		owner: sync::Arc<dyn Owner>,
		internal: backend::vk::Image,
		allocation_handle: Option<vk_mem::Allocation>,
		allocation_info: Option<vk_mem::AllocationInfo>,
		image_info: Option<image::Builder>,
	) -> Image {
		Image {
			owner: Some(owner),
			internal,
			allocation_handle,
			allocation_info,
			image_info,
		}
	}
}

impl std::ops::Deref for Image {
	type Target = backend::vk::Image;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		if let Some(owner) = &self.owner {
			owner
				.destroy(self, self.allocation_handle.as_ref())
				.unwrap();
		}
	}
}

impl Image {
	pub fn image_size(&self) -> Vector<usize, 3> {
		self.image_info.as_ref().unwrap().size()
	}

	pub fn format(&self) -> flags::Format {
		self.image_info.as_ref().unwrap().format
	}

	pub fn memory_size(&self) -> usize {
		self.allocation_info.as_ref().unwrap().get_size()
	}
}
