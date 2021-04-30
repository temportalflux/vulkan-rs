use crate::{
	alloc, backend, buffer,
	flags::{BufferUsage, MemoryProperty, MemoryUsage, SharingMode},
	utility::{self, VulkanObject},
};
use std::rc::Rc;

pub struct Buffer {
	internal: backend::vk::Buffer,
	allocation_handle: Rc<vk_mem::Allocation>,
	allocation_info: vk_mem::AllocationInfo,
	allocator: Rc<alloc::Allocator>,
}

impl Buffer {
	pub fn builder() -> buffer::Builder {
		buffer::Builder::default()
	}

	pub fn from(
		allocator: Rc<alloc::Allocator>,
		internal: backend::vk::Buffer,
		allocation_handle: vk_mem::Allocation,
		allocation_info: vk_mem::AllocationInfo,
	) -> Buffer {
		Buffer {
			allocator,
			internal,
			allocation_handle: Rc::new(allocation_handle),
			allocation_info,
		}
	}

	pub fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.allocation_info.get_device_memory().as_raw()
	}

	pub fn create_staging(
		size: usize,
		allocator: &Rc<alloc::Allocator>,
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
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Buffer`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::Buffer> for Buffer {
	fn unwrap(&self) -> &backend::vk::Buffer {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Buffer {
		&mut self.internal
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		utility::as_alloc_error(
			self.allocator
				.unwrap()
				.destroy_buffer(*self.unwrap(), &self.allocation_handle),
		)
		.unwrap();
	}
}

impl alloc::Object for Buffer {
	fn info(&self) -> &vk_mem::AllocationInfo {
		&self.allocation_info
	}

	fn handle(&self) -> &Rc<vk_mem::Allocation> {
		&self.allocation_handle
	}

	fn allocator(&self) -> &Rc<alloc::Allocator> {
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
