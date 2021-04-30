use crate::{
	alloc, backend,
	buffer::Buffer,
	flags::{BufferUsage, SharingMode},
	utility::{self, VulkanInfo, VulkanObject},
};
use std::rc::Rc;

pub struct Builder {
	pub mem_info: alloc::Info,
	pub size: u64,
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

	pub fn with_size(mut self, size: u64) -> Self {
		self.size = size;
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

impl utility::VulkanInfo<backend::vk::BufferCreateInfo> for Builder {
	/// Converts the [`Builder`] into the [`backend::vk::ImageCreateInfo`] struct
	/// used to create a [`image::Image`].
	fn to_vk(&self) -> backend::vk::BufferCreateInfo {
		backend::vk::BufferCreateInfo::builder()
			.size(self.size)
			.usage(self.usage)
			.sharing_mode(self.sharing_mode)
			.queue_family_indices(&self.queue_families[..])
			.build()
	}
}

impl Builder {
	/// Creates an [`object::Buffer`] object, thereby consuming the info.
	pub fn build(self, allocator: &Rc<alloc::Allocator>) -> utility::Result<Buffer> {
		let buffer_info = self.to_vk();
		let alloc_create_info = self.mem_info.to_vk();
		let (internal, alloc_handle, alloc_info) = utility::as_alloc_error(
			allocator
				.unwrap()
				.create_buffer(&buffer_info, &alloc_create_info),
		)?;
		Ok(Buffer::from(
			allocator.clone(),
			internal,
			alloc_handle,
			alloc_info,
		))
	}
}
