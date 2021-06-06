//! Structs which handle describing barriers sent to [`mark_pipeline_barrier`](crate::command::Buffer::mark_pipeline_barrier).

use crate::{
	backend, buffer,
	flags::{Access, ImageLayout, PipelineStage},
	image,
	structs::subresource,
};
use enumset::EnumSet;
use std::sync;

/// A collection of barriers for a pipeline which is created to [`mark a pipeline barrier`](crate::command::Buffer::mark_pipeline_barrier).
pub struct Pipeline {
	pub(crate) src_stage: PipelineStage,
	pub(crate) dst_stage: PipelineStage,
	pub(crate) kinds: Vec<Kind>,
}

impl Pipeline {
	pub fn new(src: PipelineStage, dst: PipelineStage) -> Self {
		Self {
			src_stage: src,
			dst_stage: dst,
			kinds: Vec::new(),
		}
	}

	/// Includes a specific barrier kind in the barriers being marked.
	pub fn with(mut self, kind: Kind) -> Self {
		self.kinds.push(kind);
		self
	}
}

/// The kinds of barriers that can be sent to [`mark_pipeline_barrier`](crate::command::Buffer::mark_pipeline_barrier).
pub enum Kind {
	/// A barrier around a specific segment of memory.
	Memory(Memory),
	/// A barrier for a specific buffer object.
	Buffer(Buffer),
	/// A barrier for a specific image object.
	Image(Image),
}

/// Defines the access masks that create a [`memory barrier`](Kind::Memory).
pub struct Memory {
	src_access: EnumSet<Access>,
	dst_access: EnumSet<Access>,
}

impl Default for Memory {
	fn default() -> Self {
		Self {
			src_access: EnumSet::empty(),
			dst_access: EnumSet::empty(),
		}
	}
}

impl Into<backend::vk::MemoryBarrier> for Memory {
	fn into(self) -> backend::vk::MemoryBarrier {
		backend::vk::MemoryBarrier::builder()
			.src_access_mask(Access::fold(&self.src_access))
			.dst_access_mask(Access::fold(&self.dst_access))
			.build()
	}
}

/// Defines the access masks, queues, and buffer data that creates a [`buffer barrier`](Kind::Buffer).
pub struct Buffer {
	src_access: EnumSet<Access>,
	src_queue_family: u32,

	dst_access: EnumSet<Access>,
	dst_queue_family: u32,

	buffer: sync::Weak<buffer::Buffer>,
	offset: usize,
	size: usize,
}

impl Default for Buffer {
	fn default() -> Self {
		Self {
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

impl Buffer {
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

/// Defines the access masks, queues, and image data that creates a [`image barrier`](Kind::Image).
pub struct Image {
	src_access: EnumSet<Access>,
	src_queue_family: u32,

	dst_access: EnumSet<Access>,
	dst_queue_family: u32,

	image: sync::Weak<image::Image>,
	old_layout: ImageLayout,
	new_layout: ImageLayout,
	range: subresource::Range,
}

impl Default for Image {
	fn default() -> Self {
		Self {
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

impl Image {
	/// Includes a provided access mask that is required for the source access.
	pub fn requires(mut self, access: Access) -> Self {
		self.src_access |= access;
		self
	}

	/// Includes a provided access mask that is required for the destination access.
	pub fn prevents(mut self, access: Access) -> Self {
		self.dst_access |= access;
		self
	}

	/// Includes a pointer to an image in the barrier.
	pub fn with_image(mut self, image: sync::Weak<image::Image>) -> Self {
		self.image = image;
		self
	}

	/// Indicates the layout change that will happen for the provided image.
	pub fn with_layout(mut self, prev: ImageLayout, next: ImageLayout) -> Self {
		self.old_layout = prev;
		self.new_layout = next;
		self
	}

	/// Sets the area of the image that is affected.
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
