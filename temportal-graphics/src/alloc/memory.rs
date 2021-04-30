use crate::{
	alloc,
	utility::{self, VulkanObject},
};
use std::{io::Write, rc::Rc};

pub trait Object {
	fn info(&self) -> &vk_mem::AllocationInfo;
	fn handle(&self) -> &Rc<vk_mem::Allocation>;
	fn allocator(&self) -> &Rc<alloc::Allocator>;
}

pub struct Memory {
	ptr: *mut u8,
	size: usize,
	amount_written: usize,
	handle: Rc<vk_mem::Allocation>,
	allocator: Rc<alloc::Allocator>,
}

impl Memory {
	pub fn new(obj: &impl Object) -> utility::Result<Memory> {
		Ok(Memory {
			ptr: utility::as_alloc_error(obj.allocator().unwrap().map_memory(&obj.handle()))?,
			size: obj.info().get_size(),
			amount_written: 0,
			handle: obj.handle().clone(),
			allocator: obj.allocator().clone(),
		})
	}
}

impl Drop for Memory {
	fn drop(&mut self) {
		utility::as_alloc_error(self.allocator.unwrap().unmap_memory(&self.handle)).unwrap();
	}
}

impl Write for Memory {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		let copy_size = buf.len().min(self.size - self.amount_written);
		unsafe { std::ptr::copy(buf.as_ptr(), self.ptr, copy_size) }
		self.amount_written += copy_size;
		Ok(copy_size)
	}
	fn flush(&mut self) -> std::io::Result<()> {
		self.allocator
			.unwrap()
			.flush_allocation(&self.handle, 0, self.size)
			.unwrap();
		Ok(())
	}
}
