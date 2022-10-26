use crate::{
	buffer::Buffer,
	utility::{self},
};
use std::{io::Write, sync};

/// A chunk of memory that is writable on the CPU, and may be accessible on the GPU.
///
/// Predominately used internally to handle writing to allocator [`objects`](Object).
pub struct Memory {
	size: usize,
	amount_written: usize,
	buffer: sync::Arc<Buffer>,
}

impl Memory {
	/// Starts a memory mapping to the memory for a given object.
	/// Mapping will be unmapped when the memory wrapper is dropped.
	pub fn new(buffer: sync::Arc<Buffer>) -> utility::Result<Memory> {
		let size = buffer.size();
		Ok(Memory {
			size,
			amount_written: 0,
			buffer,
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
		if let Some(mapped) = self.buffer.handle().mapped_ptr() {
			let src = buf.as_ptr() as *const u8;
			let dst = mapped.as_ptr() as *mut u8;
			let dst = ((dst as usize) + self.amount_written) as *mut u8;
			unsafe { std::ptr::copy(src, dst, buf_size) };
		}
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
		let buf_size = std::mem::size_of::<T>();
		if buf_size > self.size - self.amount_written {
			return Ok(false);
		}
		if let Some(mapped) = self.buffer.handle().mapped_ptr() {
			let src = (item as *const T) as *const u8;
			let dst = mapped.as_ptr() as *mut u8;
			let dst = ((dst as usize) + self.amount_written) as *mut u8;
			unsafe { std::ptr::copy(src, dst, buf_size) };
		}
		self.amount_written += buf_size;
		Ok(true)
	}
}

impl Write for Memory {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		Ok(match self.write_slice(buf)? {
			true => buf.len(),
			false => 0,
		})
	}

	fn flush(&mut self) -> std::io::Result<()> {
		Ok(())
	}
}
