use crate::{
	alloc, backend,
	buffer::Builder,
	flags::{BufferUsage, IndexType, MemoryLocation, SharingMode},
	utility::{self, HandledObject},
};
use std::sync;

/// A vulkan object created by the [`Allocator`](alloc::Allocator) which can store
/// ✨data✨ on the CPU and/or GPU.
///
/// Frequently used for writing things like vertices/indices and uniforms to shaders.
///
/// When a `Buffer` object is dropped, the allocation on the GPU is also dropped, thereby destroying the buffer.
pub struct Buffer {
	/// The vulkan api buffer (which does not tend to follow rust semantics).
	/// If it is dropped, the buffer wont actually be destroyed
	/// (which is why this object has a handle to the allocator and alloc info).
	internal: backend::vk::Buffer,
	allocation_handle: Option<gpu_allocator::vulkan::Allocation>,
	allocator: sync::Arc<alloc::Allocator>,
	builder: Builder,
}

impl Buffer {
	/// Helper method for creating a default buffer builder.
	pub fn builder() -> Builder {
		Builder::default()
	}

	/// Constructs the buffer object from a completed [`Builder`].
	pub(crate) fn from(
		allocator: sync::Arc<alloc::Allocator>,
		internal: backend::vk::Buffer,
		allocation_handle: gpu_allocator::vulkan::Allocation,
		builder: Builder,
	) -> Buffer {
		Buffer {
			allocator,
			internal,
			allocation_handle: Some(allocation_handle),
			builder,
		}
	}

	/// Creates an [`exclusive`](SharingMode::EXCLUSIVE) buffer,
	/// on [`only the GPU`](MemoryLocation::GpuOnly),
	/// with a given size, that can be [`transfered to`](BufferUsage::TRANSFER_DST).
	pub fn create_gpu(
		name: String,
		allocator: &sync::Arc<alloc::Allocator>,
		usage: BufferUsage,
		size: usize,
		index_type: Option<IndexType>,
		supress_log_on_drop: bool,
	) -> anyhow::Result<sync::Arc<Self>> {
		use utility::{BuildFromAllocator, NameableBuilder};
		let mut builder = Self::builder()
			.with_name(name)
			.with_sharing(SharingMode::EXCLUSIVE)
			.with_usage(usage)
			.with_usage(BufferUsage::TRANSFER_DST)
			.with_index_type(index_type)
			.with_size(size)
			.with_location(MemoryLocation::GpuOnly);
		if supress_log_on_drop {
			builder = builder.supress_log_on_drop();
		}
		Ok(sync::Arc::new(builder.build(&allocator)?))
	}

	/// Creates an [`exclusive`](SharingMode::EXCLUSIVE) buffer,
	/// which can be written from [`CPU to GPU`](MemoryLocation::CpuToGpu),
	/// with a given size, that can be [`transfered from`](BufferUsage::TRANSFER_SRC).
	pub fn create_staging(
		name: String,
		allocator: &sync::Arc<alloc::Allocator>,
		size: usize,
	) -> anyhow::Result<Self> {
		use utility::{BuildFromAllocator, NameableBuilder};
		Self::builder()
			.with_name(name)
			.with_sharing(SharingMode::EXCLUSIVE)
			.with_usage(BufferUsage::TRANSFER_SRC)
			.with_size(size)
			.with_location(MemoryLocation::CpuToGpu)
			.supress_log_on_drop()
			.build(allocator)
	}

	/// Expands the buffer to hold a `required_capacity`.
	/// If the size of the buffer can already support `required_capacity`, then no changes are made.
	/// Otherwise, an allocation resize is attempted. If the allocation cannot be extended
	/// (which is always the case due to an allocator bug),
	/// then a new buffer is allocated wih the desired capacity.
	/// Returns None if no new buffer is required. Otherwise, returns the result of creating the new buffer.
	pub fn expand(&self, required_capacity: usize) -> Option<anyhow::Result<Buffer>> {
		use utility::BuildFromAllocator;
		if self.builder.size() < required_capacity {
			let mut builder = self.builder.clone();
			builder.set_size(required_capacity);
			return Some(builder.build(&self.allocator));
		}
		None
	}

	pub(crate) fn handle(&self) -> &gpu_allocator::vulkan::Allocation {
		self.allocation_handle.as_ref().unwrap()
	}

	pub fn size(&self) -> usize {
		self.builder.size()
	}

	/// Maps the memory of the buffer for writing.
	/// The buffer must be CPU visibile/mappable in order for this to succeed.
	/// Returns the [`Memory`](alloc::Memory) mapping for writing,
	/// which will become unmapped when the memory alloc object is dropped.
	pub fn memory(self: sync::Arc<Self>) -> utility::Result<alloc::Memory> {
		alloc::Memory::new(self)
	}

	pub fn index_type(&self) -> &Option<IndexType> {
		self.builder.index_type()
	}

	pub fn rename(&self, name: &str) {
		// NOTE: Intentionally does not modify the builder because that would mean marking the buffer as mutable
		// and there is no use case for a persistent name change (rename and then expand the same buffer).
		// When that becomes a need, the stored builder will need interiorn mutability.
		if let Some(device) = self.allocator.logical() {
			device.set_object_name_logged(&self.create_name(name));
		}
	}
}

impl std::ops::Deref for Buffer {
	type Target = backend::vk::Buffer;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		if !self.builder.supress_drop_log {
			use utility::NameableBuilder;
			log::debug!(
				target: crate::LOG,
				"Dropping Buffer: {:?}",
				self.builder.name()
			);
		}
		let allocation = self.allocation_handle.take().unwrap();
		self.allocator.destroy_buffer(**self, allocation).unwrap();
	}
}

impl utility::HandledObject for Buffer {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Buffer as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}

impl utility::NamedObject for Buffer {
	fn name(&self) -> &String {
		use utility::NameableBuilder;
		self.builder.name()
	}
}
