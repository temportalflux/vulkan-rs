use crate::{alloc, backend, flags, image, utility};
use std::sync;
use temportal_math::Vector;

pub trait Owner: Send + Sync {
	fn destroy(&self, obj: &Image, allocation: Option<&vk_mem::Allocation>) -> utility::Result<()>;
}

/// An handle representing image data stored on the [`GPU`](crate::device::physical::Device),
/// including any created by the [`Swapchain`](crate::device::swapchain::Swapchain).
pub struct Image {
	image_info: Option<image::Builder>,
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
		image_info: Option<image::Builder>,
	) -> Image {
		Image {
			owner: Some(owner),
			internal,
			allocation_handle,
			image_info,
		}
	}

	pub fn create_gpu(
		allocator: &sync::Arc<alloc::Allocator>,
		format: flags::Format,
		size: Vector<usize, 3>,
	) -> utility::Result<Self> {
		Ok(Self::builder()
			.with_alloc(
				alloc::Builder::default()
					.with_usage(flags::MemoryUsage::GpuOnly)
					.requires(flags::MemoryProperty::DEVICE_LOCAL),
			)
			.with_format(format)
			.with_size(size)
			.with_usage(flags::ImageUsage::TRANSFER_DST)
			.with_usage(flags::ImageUsage::SAMPLED)
			.build(allocator)?)
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
}
