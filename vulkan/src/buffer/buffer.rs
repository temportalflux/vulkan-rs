use crate::{
	alloc, backend,
	buffer::Builder,
	flags::{BufferUsage, IndexType, MemoryProperty, MemoryUsage, SharingMode},
	utility::{self},
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
	allocation_handle: sync::Arc<vk_mem::Allocation>,
	allocation_info: vk_mem::AllocationInfo,
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
		allocation_handle: vk_mem::Allocation,
		allocation_info: vk_mem::AllocationInfo,
		builder: Builder,
	) -> Buffer {
		Buffer {
			allocator,
			internal,
			allocation_handle: sync::Arc::new(allocation_handle),
			allocation_info,
			builder,
		}
	}

	/// Creates an [`exclusive`](SharingMode::EXCLUSIVE) buffer,
	/// on [`only the GPU`](MemoryUsage::GpuOnly),
	/// with a given size, that can be [`transfered to`](BufferUsage::TRANSFER_DST).
	pub fn create_gpu(
		name: Option<String>,
		allocator: &sync::Arc<alloc::Allocator>,
		usage: BufferUsage,
		size: usize,
		index_type: Option<IndexType>,
	) -> utility::Result<sync::Arc<Self>> {
		use utility::{BuildFromAllocator, NameableBuilder};
		Ok(sync::Arc::new(
			Self::builder()
				.with_optname(name)
				.with_sharing(SharingMode::EXCLUSIVE)
				.with_usage(usage)
				.with_usage(BufferUsage::TRANSFER_DST)
				.with_index_type(index_type)
				.with_size(size)
				.with_alloc(
					alloc::Builder::default()
						.with_usage(MemoryUsage::GpuOnly)
						.requires(MemoryProperty::DEVICE_LOCAL),
				)
				.build(&allocator)?,
		))
	}

	/// Creates an [`exclusive`](SharingMode::EXCLUSIVE) buffer,
	/// which can be written from [`CPU to GPU`](MemoryUsage::CpuToGpu),
	/// with a given size, that can be [`transfered from`](BufferUsage::TRANSFER_SRC).
	pub fn create_staging(
		name: Option<String>,
		allocator: &sync::Arc<alloc::Allocator>,
		size: usize,
	) -> utility::Result<Self> {
		use utility::{BuildFromAllocator, NameableBuilder};
		Self::builder()
			.with_optname(name)
			.with_sharing(SharingMode::EXCLUSIVE)
			.with_usage(BufferUsage::TRANSFER_SRC)
			.with_size(size)
			.with_alloc(
				alloc::Builder::default()
					.with_usage(MemoryUsage::CpuToGpu)
					.requires(MemoryProperty::HOST_VISIBLE)
					.requires(MemoryProperty::HOST_COHERENT),
			)
			.build(allocator)
	}

	/// Attempts to change the allocated memory to a new size.
	/// Returns false if the resize failed.
	fn resize_allocation(&mut self, _new_size: usize) -> bool {
		// TODO: Can produce an error if resize is successful
		// where the internal backend buffer still believes its the old size.
		false
		//let success = self
		//	.allocator
		//	.resize_allocation(&self.allocation_handle, new_size)
		//	.is_ok();
		//if success {
		//	self.builder.set_size(new_size);
		//}
		//success
	}

	/// Expands the buffer to hold a `required_capacity`.
	/// If the size of the buffer can already support `required_capacity`, then no changes are made.
	/// Otherwise, an allocation resize is attempted. If the allocation cannot be extended,
	/// then a new buffer is allocated wih the desired capacity.
	pub fn expand(&mut self, required_capacity: usize) -> utility::Result<()> {
		use alloc::Object;
		if self.size() < required_capacity {
			self.builder.set_size(required_capacity);
			if !self.resize_allocation(required_capacity) {
				let (raw, handle, info) = self.builder.rebuild(&self.allocator())?;
				self.internal = raw;
				self.allocation_handle = sync::Arc::new(handle);
				self.allocation_info = info;
			}
		}
		Ok(())
	}

	/// Maps the memory of the buffer for writing.
	/// The buffer must be CPU visibile/mappable in order for this to succeed.
	/// Returns the [`Memory`](alloc::Memory) mapping for writing,
	/// which will become unmapped when the memory alloc object is dropped.
	pub fn memory(&self) -> utility::Result<alloc::Memory> {
		alloc::Memory::new(self)
	}

	pub fn index_type(&self) -> &Option<IndexType> {
		self.builder.index_type()
	}

	pub fn name(&self) -> &Option<String> {
		use utility::NameableBuilder;
		self.builder.name()
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
		self.allocator
			.destroy_buffer(**self, &self.allocation_handle)
			.unwrap();
	}
}

impl alloc::Object for Buffer {
	fn size(&self) -> usize {
		self.builder.size()
	}

	fn info(&self) -> &vk_mem::AllocationInfo {
		&self.allocation_info
	}

	fn handle(&self) -> &sync::Arc<vk_mem::Allocation> {
		&self.allocation_handle
	}

	fn allocator(&self) -> &sync::Arc<alloc::Allocator> {
		&self.allocator
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
