use crate::{
	alloc,
	utility::{self},
};
use std::{io::Write, sync};

/// Trait for marking types which represent data which is allocated by an [`allocator`](alloc::Allocator).
///
/// All allocated objects have a size, allocation info, allocation handle,
/// and should retain a reference to their allocator to ensure proper memory dropping order.
pub trait Object {
	fn size(&self) -> usize;
	fn info(&self) -> &vk_mem::AllocationInfo;
	fn handle(&self) -> &sync::Arc<vk_mem::Allocation>;
	fn allocator(&self) -> &sync::Arc<alloc::Allocator>;
}

/// A chunk of memory that is writable on the CPU, and may be accessible on the GPU.
///
/// Predominately used internally to handle writing to allocator [`objects`](Object).
pub struct Memory {
	ptr: *mut u8,
	size: usize,
	amount_written: usize,
	handle: sync::Arc<vk_mem::Allocation>,
	allocator: sync::Arc<alloc::Allocator>,
}

impl Memory {
	/// Starts a memory mapping to the memory for a given object.
	/// Mapping will be unmapped when the memory wrapper is dropped.
	pub fn new(obj: &impl Object) -> utility::Result<Memory> {
		Ok(Memory {
			ptr: obj.allocator().map_memory(&obj.handle())?,
			size: obj.size(),
			amount_written: 0,
			handle: obj.handle().clone(),
			allocator: obj.allocator().clone(),
		})
	}

	pub fn amount_written(&self) -> usize {
		self.amount_written
	}

	/// Writes a slice of any type to the object's memory,
	/// and results in a `false` if the slice would result in
	/// an overflow beyond the size of an object.
	///
	/// Multiple items can be written in sequence,
	/// so long as the total memory does not overflow the object's size.
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

	/// Writes an object of any type to the object's memory,
	/// and results in a `false` if the slice would result in
	/// an overflow beyond the size of an object.
	///
	/// Multiple items can be written in sequence,
	/// so long as the total memory does not overflow the object's size.
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
		self.allocator.unmap_memory(&self.handle);
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
		self.allocator.flush_allocation(&self.handle, 0, self.size);
		Ok(())
	}
}
