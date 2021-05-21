use crate::{
	alloc, backend,
	buffer::Buffer,
	flags::{BufferUsage, SharingMode},
	utility,
};
use std::sync;

/// The builder for [`Buffer`] objects.
#[derive(Clone)]
pub struct Builder {
	/// The allocation information/builder for allocating the buffer.
	mem_info: alloc::Builder,
	/// The desired size of the buffer.
	size: usize,
	/// What kind of buffer to create / how the buffer will be used.
	usage: BufferUsage,
	sharing_mode: SharingMode,
	queue_families: Vec<u32>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			mem_info: alloc::Builder::default(),
			size: 0,
			usage: BufferUsage::empty(),
			sharing_mode: SharingMode::EXCLUSIVE,
			queue_families: Vec::new(),
		}
	}
}

impl Builder {
	/// Mutates the builder to include the memory allocation information.
	pub fn with_alloc(mut self, mem_info: alloc::Builder) -> Self {
		self.mem_info = mem_info;
		self
	}

	/// Mutates the builder to include an explicit size that
	/// will be the capacity for the buffer object.
	pub fn with_size(mut self, size: usize) -> Self {
		self.set_size(size);
		self
	}

	/// Sets the explicit size that
	/// will be the capacity for the buffer object.
	pub fn set_size(&mut self, size: usize) {
		self.size = size;
	}

	/// Mutates the builder to include a buffer capacity that can
	/// support a specific set of data.
	pub fn with_size_of<T: Sized>(mut self, slice: &[T]) -> Self {
		self.size = std::mem::size_of::<T>() * slice.len();
		self
	}

	pub(crate) fn size(&self) -> usize {
		self.size
	}

	/// Mutates the builder to include a flag indicating how the buffer will be used.
	/// Can be called multiple times with different flags to include each flag.
	pub fn with_usage(mut self, usage: BufferUsage) -> Self {
		self.usage |= usage;
		self
	}

	/// Mutates the builder to include a specific sharing mode.
	/// The sharing mode is [`exclusive`](SharingMode::EXCLUSIVE) by default.
	pub fn with_sharing(mut self, mode: SharingMode) -> Self {
		self.sharing_mode = mode;
		self
	}

	/// Mutates the builder to include a specific queue family.
	/// Can be called multiple times to support multiple queue families.
	pub fn with_queue(mut self, family_index: usize) -> Self {
		self.queue_families.push(family_index as u32);
		self
	}

	/// Creates a [`Buffer`] object, thereby consuming the info.
	pub fn build(self, allocator: &sync::Arc<alloc::Allocator>) -> utility::Result<Buffer> {
		let (internal, alloc_handle, alloc_info) = self.rebuild(&allocator)?;
		Ok(Buffer::from(
			allocator.clone(),
			internal,
			alloc_handle,
			alloc_info,
			self,
		))
	}

	/// Used by [`Buffer`] to re-allocate a buffer object when resizing/expanding the allocation.
	pub(crate) fn rebuild(
		&self,
		allocator: &alloc::Allocator,
	) -> utility::Result<(ash::vk::Buffer, vk_mem::Allocation, vk_mem::AllocationInfo)> {
		let buffer_info = backend::vk::BufferCreateInfo::builder()
			.size(self.size as u64)
			.usage(self.usage)
			.sharing_mode(self.sharing_mode)
			.queue_family_indices(&self.queue_families[..])
			.build();
		let alloc_create_info = self.mem_info.clone().into();
		utility::as_alloc_error(allocator.create_buffer(&buffer_info, &alloc_create_info))
	}
}
