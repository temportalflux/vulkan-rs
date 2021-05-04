use crate::{
	alloc,
	utility::{self, VulkanObject},
};
use std::{io::Write, sync};

pub trait Object {
	fn size(&self) -> usize;
	fn info(&self) -> &vk_mem::AllocationInfo;
	fn handle(&self) -> &sync::Arc<vk_mem::Allocation>;
	fn allocator(&self) -> &sync::Arc<alloc::Allocator>;
}

pub struct Memory {
	ptr: *mut u8,
	size: usize,
	amount_written: usize,
	handle: sync::Arc<vk_mem::Allocation>,
	allocator: sync::Arc<alloc::Allocator>,
}

impl Memory {
	pub fn new(obj: &impl Object) -> utility::Result<Memory> {
		Ok(Memory {
			ptr: obj.allocator().unwrap().map_memory(&obj.handle())?,
			size: obj.size(),
			amount_written: 0,
			handle: obj.handle().clone(),
			allocator: obj.allocator().clone(),
		})
	}

	pub fn write_slice<T: Sized>(&mut self, buf: &[T]) -> std::io::Result<bool> {
		let buf_size = std::mem::size_of::<T>() * buf.len();
		if buf_size > self.size - self.amount_written {
			return Ok(false);
		}
		/*
		log::debug!(
			"writing {} bytes to {:#x} at pos {} and max size {}",
			buf_size,
			self.ptr as u64,
			self.amount_written,
			self.size
		);
		*/
		let src = buf.as_ptr() as *const u8;
		let dst: *mut u8 = ((self.ptr as usize) + self.amount_written) as *mut u8;
		unsafe { std::ptr::copy(src, dst, buf_size) }
		self.amount_written += buf_size;
		Ok(true)
	}

	pub fn write_item<T: Sized>(&mut self, item: &T) -> std::io::Result<bool> {
		let item_size = std::mem::size_of::<T>();
		if item_size > self.size - self.amount_written {
			return Ok(false);
		}
		/*
		log::debug!(
			"writing {} bytes to {:#x} at pos {} and max size {}",
			buf_size,
			self.ptr as u64,
			self.amount_written,
			self.size
		);
		*/
		let src = (item as *const T) as *const u8;
		let dst: *mut u8 = ((self.ptr as usize) + self.amount_written) as *mut u8;
		unsafe { std::ptr::copy(src, dst, item_size) }
		self.amount_written += item_size;
		Ok(true)
	}
}

impl Drop for Memory {
	fn drop(&mut self) {
		self.allocator.unwrap().unmap_memory(&self.handle).unwrap();
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
