use crate::{
	alloc, backend,
	flags::{format::Format, ImageUsage, MemoryLocation},
	image::Builder,
	structs::{Extent2D, Extent3D},
	utility::{self, NameableBuilder},
};
use std::sync;

pub(crate) trait Owner: Send + Sync {
	fn destroy(
		&self,
		obj: &Image,
		allocation: Option<gpu_allocator::vulkan::Allocation>,
	) -> anyhow::Result<()>;
}

/// An handle representing image data stored on the [`GPU`](crate::device::physical::Device),
/// including any created by the [`Swapchain`](crate::device::swapchain::khr::Swapchain).
///
/// When an `Image` object is dropped, the allocation on the GPU is also dropped, thereby destroying the image.
pub struct Image {
	dimensions: Extent3D,
	format: Format,
	allocation_handle: Option<gpu_allocator::vulkan::Allocation>,
	internal: backend::vk::Image,
	owner: Option<sync::Arc<dyn Owner>>, // empty for images created from the swapchain
	name: String,
}

impl Image {
	/// Internal method for creating the image from a provided vulkan image from the [`Swapchain`](crate::device::swapchain::khr::Swapchain).
	pub(crate) fn from_swapchain(
		internal: backend::vk::Image,
		name: String,
		format: Format,
		dimensions: Extent2D,
	) -> Image {
		Image {
			owner: None,
			internal,
			allocation_handle: None,
			format: format,
			dimensions: Extent3D {
				width: dimensions.width,
				height: dimensions.height,
				depth: 1,
			},
			name,
		}
	}

	/// Helper method for creating a default image builder.
	pub fn builder() -> Builder {
		Builder::default()
	}

	/// Internal method for constructing the image object from a completed [`Builder`].
	pub(crate) fn new(
		owner: sync::Arc<dyn Owner>,
		internal: backend::vk::Image,
		allocation_handle: Option<gpu_allocator::vulkan::Allocation>,
		image_info: Builder,
	) -> Image {
		Image {
			owner: Some(owner),
			internal,
			allocation_handle,
			dimensions: image_info.size(),
			format: image_info.format(),
			name: image_info.name().clone(),
		}
	}

	/// Creates a [`samplable`](ImageUsage::SAMPLED) image,
	/// on [`only the GPU`](MemoryLocation::GpuOnly),
	/// with a given size & format, that can be [`transfered to`](ImageUsage::TRANSFER_DST).
	pub fn create_gpu(
		allocator: &sync::Arc<alloc::Allocator>,
		name: String,
		format: Format,
		size: Extent3D,
	) -> anyhow::Result<Self> {
		use utility::BuildFromAllocator;
		Ok(Self::builder()
			.with_name(name)
			.with_location(MemoryLocation::GpuOnly)
			.with_format(format)
			.with_size(size)
			.with_usage(ImageUsage::TRANSFER_DST)
			.with_usage(ImageUsage::SAMPLED)
			.build(allocator)?)
	}

	/// The dimensions of the image allocated.
	pub fn image_size(&self) -> Extent3D {
		self.dimensions
	}

	/// The format of the image allocated.
	pub fn format(&self) -> Format {
		self.format
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
			let allocation = self.allocation_handle.take();
			owner.destroy(self, allocation).unwrap();
		}
	}
}

impl utility::HandledObject for Image {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Image as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}

impl utility::NamedObject for Image {
	fn name(&self) -> &String {
		&self.name
	}
}
