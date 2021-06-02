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

/// Information about an image attached to a ['Render Pass'](crate::renderpass::Pass).
/// Most frequent use is to describe the ['Swapchain'](crate::device::swapchain::Swapchain)
/// images used for each frame that is shown.
#[derive(Clone, Default)]
pub struct Attachment {
	format: flags::format::Format,
	samples: flags::SampleCount,
	general_ops: AttachmentOps,
	stencil_ops: AttachmentOps,
	initial_layout: flags::ImageLayout,
	final_layout: flags::ImageLayout,
}

impl Attachment {
	pub fn set_format(mut self, format: flags::format::Format) -> Self {
		self.format = format;
		self
	}

	pub fn set_sample_count(mut self, count: flags::SampleCount) -> Self {
		self.samples = count;
		self
	}

	pub fn set_general_ops(mut self, ops: AttachmentOps) -> Self {
		self.general_ops = ops;
		self
	}

	pub fn set_stencil_ops(mut self, ops: AttachmentOps) -> Self {
		self.stencil_ops = ops;
		self
	}

	pub fn set_initial_layout(mut self, layout: flags::ImageLayout) -> Self {
		self.initial_layout = layout;
		self
	}

	pub fn set_final_layout(mut self, layout: flags::ImageLayout) -> Self {
		self.final_layout = layout;
		self
	}
}

impl Into<backend::vk::AttachmentDescription> for Attachment {
	fn into(self) -> backend::vk::AttachmentDescription {
		backend::vk::AttachmentDescription::builder()
			.format(self.format)
			.samples(self.samples.into())
			.load_op(self.general_ops.load.into())
			.store_op(self.general_ops.store.into())
			.stencil_load_op(self.stencil_ops.load.into())
			.stencil_store_op(self.stencil_ops.store.into())
			.initial_layout(self.initial_layout.into())
			.final_layout(self.final_layout.into())
			.build()
	}
}
