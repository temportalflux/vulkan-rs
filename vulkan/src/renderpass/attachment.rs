use crate::{backend, flags};
use serde::{Deserialize, Serialize};

/// The load and store operations that can be performed on an image
/// that is attached to a ['Render Pass'](crate::renderpass::Pass)
/// and its ['Subpasses'](crate::renderpass::Subpass).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct AttachmentOps {
	pub load: flags::LoadOp,
	pub store: flags::StoreOp,
}

#[derive(Clone, Copy, Debug)]
pub enum SampleKind {
	Fixed(flags::SampleCount),
	MaxCommon,
}
impl SampleKind {
	pub fn unwrap_or(self, max_common: flags::SampleCount) -> flags::SampleCount {
		match self {
			Self::Fixed(count) => count,
			Self::MaxCommon => max_common,
		}
	}
}

/// Information about an image attached to a ['Render Pass'](crate::renderpass::Pass).
/// Most frequent use is to describe the ['Swapchain'](crate::device::swapchain::khr::Swapchain)
/// images used for each frame that is shown.
#[derive(Debug, Clone)]
pub struct Attachment {
	id: String,
	pub(crate) format: flags::format::Format,
	samples: SampleKind,
	general_ops: AttachmentOps,
	stencil_ops: AttachmentOps,
	initial_layout: flags::ImageLayout,
	final_layout: flags::ImageLayout,
}

impl Attachment {
	pub fn new(id: String) -> Self {
		Self {
			id,
			format: flags::format::default(),
			samples: SampleKind::Fixed(flags::SampleCount::_1),
			general_ops: Default::default(),
			stencil_ops: Default::default(),
			initial_layout: Default::default(),
			final_layout: Default::default(),
		}
	}

	pub fn id(&self) -> &String {
		&self.id
	}

	pub fn with_format(mut self, format: flags::format::Format) -> Self {
		self.format = format;
		self
	}

	pub fn with_sample_count(self, count: flags::SampleCount) -> Self {
		self.with_sample_kind(SampleKind::Fixed(count))
	}

	pub fn with_sample_kind(mut self, count: SampleKind) -> Self {
		self.samples = count;
		self
	}

	pub fn with_general_ops(mut self, ops: AttachmentOps) -> Self {
		self.general_ops = ops;
		self
	}

	pub fn with_stencil_ops(mut self, ops: AttachmentOps) -> Self {
		self.stencil_ops = ops;
		self
	}

	pub fn with_initial_layout(mut self, layout: flags::ImageLayout) -> Self {
		self.initial_layout = layout;
		self
	}

	pub fn with_final_layout(mut self, layout: flags::ImageLayout) -> Self {
		self.final_layout = layout;
		self
	}
}

impl Attachment {
	pub fn into_desc(
		self,
		max_common_sample_count: flags::SampleCount,
	) -> backend::vk::AttachmentDescription {
		assert_ne!(self.format, backend::vk::Format::UNDEFINED);
		backend::vk::AttachmentDescription::builder()
			.format(self.format)
			.samples(self.samples.unwrap_or(max_common_sample_count).into())
			.load_op(self.general_ops.load.into())
			.store_op(self.general_ops.store.into())
			.stencil_load_op(self.stencil_ops.load.into())
			.stencil_store_op(self.stencil_ops.store.into())
			.initial_layout(self.initial_layout.into())
			.final_layout(self.final_layout.into())
			.build()
	}
}
