use crate::{
	alloc, backend,
	buffer::Buffer,
	flags::{BufferUsage, IndexType, MemoryLocation, SharingMode},
	utility::{self, HandledObject},
};
use std::sync;

/// The builder for [`Buffer`] objects.
#[derive(Clone)]
pub struct Builder {
	name: String,
	pub(crate) supress_drop_log: bool,
	/// The allocation information/builder for allocating the buffer.
	location: MemoryLocation,
	/// The desired size of the buffer.
	size: usize,
	/// What kind of buffer to create / how the buffer will be used.
	usage: BufferUsage,
	sharing_mode: SharingMode,
	queue_families: Vec<u32>,
	index_type: Option<IndexType>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			name: String::new(),
			supress_drop_log: false,
			location: MemoryLocation::Unknown,
			size: 0,
			usage: BufferUsage::empty(),
			sharing_mode: SharingMode::EXCLUSIVE,
			queue_families: Vec::new(),
			index_type: None,
		}
	}
}

impl Builder {
	/// Mutates the builder to include the memory allocation information.
	pub fn with_location(mut self, location: MemoryLocation) -> Self {
		self.location = location;
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

	/// Sets the index type that the buffer contains.
	/// Only used/valid if using an [`index buffer`](BufferUsage::INDEX_BUFFER).
	pub fn with_index_type(mut self, kind: Option<IndexType>) -> Self {
		self.index_type = kind;
		self
	}

	/// Returns the index type if it was set by [`with_index_type`](Self::with_index_type).
	pub fn index_type(&self) -> &Option<IndexType> {
		&self.index_type
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

	pub fn supress_log_on_drop(mut self) -> Self {
		self.supress_drop_log = true;
		self
	}
}

impl utility::NameableBuilder for Builder {
	fn set_name(&mut self, name: impl Into<String>) {
		self.name = name.into();
	}

	fn name(&self) -> &String {
		&self.name
	}
}

impl utility::BuildFromAllocator for Builder {
	type Output = Buffer;
	/// Creates a [`Buffer`] object, thereby consuming the info.
	fn build(self, allocator: &sync::Arc<alloc::Allocator>) -> anyhow::Result<Self::Output> {
		let (internal, allocation) = self.rebuild(&allocator)?;
		let buffer = Buffer::from(allocator.clone(), internal, allocation, self.clone());
		allocator
			.logical()
			.unwrap()
			.set_object_name_logged(&buffer.create_name(&self.name));
		Ok(buffer)
	}
}

impl Builder {
	/// Used by [`Buffer`] to re-allocate a buffer object when resizing/expanding the allocation.
	pub(crate) fn rebuild(
		&self,
		allocator: &alloc::Allocator,
	) -> anyhow::Result<(ash::vk::Buffer, gpu_allocator::vulkan::Allocation)> {
		if self.usage.contains(BufferUsage::INDEX_BUFFER) != self.index_type.is_some() {
			Err(utility::Error::InvalidBufferFormat(
				match self.usage.contains(BufferUsage::INDEX_BUFFER) {
					true => "Index Buffers must have an index type",
					false => "Non-Index Buffers cannot have an index type",
				}
				.to_owned(),
			))?;
		}
		let buffer_info = backend::vk::BufferCreateInfo::builder()
			.size(self.size as u64)
			.usage(self.usage)
			.sharing_mode(self.sharing_mode)
			.queue_family_indices(&self.queue_families[..])
			.build();
		allocator.create_buffer(&self.name, self.location, &buffer_info)
	}
}
