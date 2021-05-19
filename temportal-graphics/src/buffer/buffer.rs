use crate::{
	alloc, backend, buffer,
	flags::{BufferUsage, MemoryProperty, MemoryUsage, SharingMode},
	utility::{self},
};
use std::sync;

pub struct Buffer {
	internal: backend::vk::Buffer,
	allocation_handle: sync::Arc<vk_mem::Allocation>,
	allocation_info: vk_mem::AllocationInfo,
	allocator: sync::Arc<alloc::Allocator>,
	builder: buffer::Builder,
}

impl Buffer {
	pub fn builder() -> buffer::Builder {
		buffer::Builder::default()
	}

	pub fn create_gpu(
		allocator: &sync::Arc<alloc::Allocator>,
		usage: BufferUsage,
		size: usize,
	) -> utility::Result<sync::Arc<buffer::Buffer>> {
		Ok(sync::Arc::new(
			Self::builder()
				.with_usage(usage)
				.with_usage(BufferUsage::TRANSFER_DST)
				.with_size(size)
				.with_alloc(
					alloc::Info::default()
						.with_usage(MemoryUsage::GpuOnly)
						.requires(MemoryProperty::DEVICE_LOCAL),
				)
				.with_sharing(SharingMode::EXCLUSIVE)
				.build(&allocator)?,
		))
	}

	pub fn from(
		allocator: sync::Arc<alloc::Allocator>,
		internal: backend::vk::Buffer,
		allocation_handle: vk_mem::Allocation,
		allocation_info: vk_mem::AllocationInfo,
		builder: buffer::Builder,
	) -> Buffer {
		Buffer {
			allocator,
			internal,
			allocation_handle: sync::Arc::new(allocation_handle),
			allocation_info,
			builder,
		}
	}

	pub fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.allocation_info.get_device_memory().as_raw()
	}

	pub fn create_staging(
		size: usize,
		allocator: &sync::Arc<alloc::Allocator>,
	) -> utility::Result<Buffer> {
		Buffer::builder()
			.with_size(size)
			.with_usage(BufferUsage::TRANSFER_SRC)
			.with_sharing(SharingMode::EXCLUSIVE)
			.with_alloc(
				alloc::Info::default()
					.with_usage(MemoryUsage::CpuToGpu)
					.requires(MemoryProperty::HOST_VISIBLE)
					.requires(MemoryProperty::HOST_COHERENT),
			)
			.build(allocator)
	}

	pub fn resize_allocation(&mut self, new_size: usize) -> bool {
		let success = self
			.allocator
			.resize_allocation(&self.allocation_handle, new_size)
			.is_ok();
		if success {
			self.builder.set_size(new_size);
		}
		success
	}

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
}

impl std::ops::Deref for Buffer {
	type Target = backend::vk::Buffer;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		utility::as_alloc_error(
			self.allocator
				.destroy_buffer(**self, &self.allocation_handle),
		)
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

impl Buffer {
	pub fn memory_size(&self) -> usize {
		self.allocation_info.get_size()
	}

	pub fn memory(&self) -> utility::Result<alloc::Memory> {
		alloc::Memory::new(self)
	}
}
