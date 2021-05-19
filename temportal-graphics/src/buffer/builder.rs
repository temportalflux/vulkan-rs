use crate::{
	alloc, backend,
	buffer::Buffer,
	flags::{BufferUsage, SharingMode},
	utility,
};
use std::sync;

pub struct Builder {
	pub mem_info: alloc::Info,
	pub size: usize,
	pub usage: BufferUsage,
	pub sharing_mode: SharingMode,
	pub queue_families: Vec<u32>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			mem_info: alloc::Info::default(),
			size: 0,
			usage: BufferUsage::empty(),
			sharing_mode: SharingMode::EXCLUSIVE,
			queue_families: Vec::new(),
		}
	}
}

impl Builder {
	pub fn with_alloc(mut self, mem_info: alloc::Info) -> Self {
		self.mem_info = mem_info;
		self
	}

	pub fn with_size(mut self, size: usize) -> Self {
		self.size = size;
		self
	}

	pub fn with_size_of<T: Sized>(mut self, slice: &[T]) -> Self {
		self.size = std::mem::size_of::<T>() * slice.len();
		self
	}

	pub fn with_usage(mut self, usage: BufferUsage) -> Self {
		self.usage |= usage;
		self
	}

	pub fn with_sharing(mut self, mode: SharingMode) -> Self {
		self.sharing_mode = mode;
		self
	}

	pub fn with_queue(mut self, family_index: usize) -> Self {
		self.queue_families.push(family_index as u32);
		self
	}
}

impl Builder {
	/// Creates an [`Buffer`] object, thereby consuming the info.
	pub fn build(self, allocator: &sync::Arc<alloc::Allocator>) -> utility::Result<Buffer> {
		let buffer_info = backend::vk::BufferCreateInfo::builder()
			.size(self.size as u64)
			.usage(self.usage)
			.sharing_mode(self.sharing_mode)
			.queue_family_indices(&self.queue_families[..])
			.build();
		let alloc_create_info = self.mem_info.clone().into();
		let (internal, alloc_handle, alloc_info) =
			utility::as_alloc_error(allocator.create_buffer(&buffer_info, &alloc_create_info))?;
		Ok(Buffer::from(
			allocator.clone(),
			internal,
			alloc_handle,
			alloc_info,
			self,
		))
	}
}
