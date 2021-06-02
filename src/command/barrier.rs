use crate::{
	backend, buffer,
	flags::{Access, ImageLayout, PipelineStage},
	image,
	structs::subresource,
};
use enumset::EnumSet;
use std::sync;

pub struct PipelineBarrier {
	pub src_stage: PipelineStage,
	pub dst_stage: PipelineStage,
	pub kinds: Vec<BarrierKind>,
}

pub enum BarrierKind {
	Memory(MemoryBarrier),
	Buffer(BufferBarrier),
	Image(ImageBarrier),
}

pub struct MemoryBarrier {
	src_access: EnumSet<Access>,
	dst_access: EnumSet<Access>,
}

impl Default for MemoryBarrier {
	fn default() -> MemoryBarrier {
		MemoryBarrier {
			src_access: EnumSet::empty(),
			dst_access: EnumSet::empty(),
		}
	}
}

impl Into<backend::vk::MemoryBarrier> for MemoryBarrier {
	fn into(self) -> backend::vk::MemoryBarrier {
		backend::vk::MemoryBarrier::builder()
			.src_access_mask(Access::fold(&self.src_access))
			.dst_access_mask(Access::fold(&self.dst_access))
			.build()
	}
}

pub struct BufferBarrier {
	src_access: EnumSet<Access>,
	src_queue_family: u32,

	dst_access: EnumSet<Access>,
	dst_queue_family: u32,

	buffer: sync::Weak<buffer::Buffer>,
	offset: usize,
	size: usize,
}

impl Default for BufferBarrier {
	fn default() -> BufferBarrier {
		BufferBarrier {
			src_access: EnumSet::empty(),
			src_queue_family: u32::MAX, // queue is ignored
			dst_access: EnumSet::empty(),
			dst_queue_family: u32::MAX, // queue is ignored
			buffer: sync::Weak::new(),
			offset: 0,
			size: 0,
		}
	}
}

impl BufferBarrier {
	pub(crate) fn as_vk(&self) -> backend::vk::BufferMemoryBarrier {
		backend::vk::BufferMemoryBarrier::builder()
			.src_access_mask(Access::fold(&self.src_access))
			.src_queue_family_index(self.src_queue_family)
			.dst_access_mask(Access::fold(&self.dst_access))
			.dst_queue_family_index(self.dst_queue_family)
			.buffer(**self.buffer.upgrade().unwrap())
			.offset(self.offset as u64)
			.size(self.size as u64)
			.build()
	}
}

pub struct ImageBarrier {
	src_access: EnumSet<Access>,
	src_queue_family: u32,

	dst_access: EnumSet<Access>,
	dst_queue_family: u32,

	image: sync::Weak<image::Image>,
	old_layout: ImageLayout,
	new_layout: ImageLayout,
	range: subresource::Range,
}

impl Default for ImageBarrier {
	fn default() -> ImageBarrier {
		ImageBarrier {
			src_access: EnumSet::empty(),
			src_queue_family: u32::MAX, // queue is ignored

			dst_access: EnumSet::empty(),
			dst_queue_family: u32::MAX, // queue is ignored

			image: sync::Weak::new(),
			old_layout: ImageLayout::default(),
			new_layout: ImageLayout::default(),
			range: subresource::Range::default(),
		}
	}
}

impl ImageBarrier {
	pub fn requires(mut self, access: Access) -> Self {
		self.src_access |= access;
		self
	}

	pub fn prevents(mut self, access: Access) -> Self {
		self.dst_access |= access;
		self
	}

	pub fn with_image(mut self, image: sync::Weak<image::Image>) -> Self {
		self.image = image;
		self
	}

	pub fn with_layout(mut self, prev: ImageLayout, next: ImageLayout) -> Self {
		self.old_layout = prev;
		self.new_layout = next;
		self
	}

	pub fn with_range(mut self, range: subresource::Range) -> Self {
		self.range = range;
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::ImageMemoryBarrier {
		backend::vk::ImageMemoryBarrier::builder()
			.src_access_mask(Access::fold(&self.src_access))
			.src_queue_family_index(self.src_queue_family)
			.dst_access_mask(Access::fold(&self.dst_access))
			.dst_queue_family_index(self.dst_queue_family)
			.image(**self.image.upgrade().unwrap())
			.old_layout(self.old_layout.into())
			.new_layout(self.new_layout.into())
			.subresource_range(self.range.into())
			.build()
	}
}
