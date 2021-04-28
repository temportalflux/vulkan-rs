use crate::{backend, flags, utility};

/// The load and store operations that can be performed on an image
/// that is attached to a ['Render Pass'](crate::renderpass::Pass)
/// and its ['Subpasses'](crate::renderpass::Subpass).
pub struct AttachmentOps {
	pub load: flags::AttachmentLoadOp,
	pub store: flags::AttachmentStoreOp,
}

impl Default for AttachmentOps {
	fn default() -> AttachmentOps {
		AttachmentOps {
			load: flags::AttachmentLoadOp::DONT_CARE,
			store: flags::AttachmentStoreOp::DONT_CARE,
		}
	}
}

/// Information about an image attached to a ['Render Pass'](crate::renderpass::Pass).
/// Most frequent use is to describe the ['Swapchain'](crate::device::swapchain::Swapchain)
/// images used for each frame that is shown.
pub struct Attachment {
	format: flags::Format,
	samples: flags::SampleCount,
	general_ops: AttachmentOps,
	stencil_ops: AttachmentOps,
	initial_layout: flags::ImageLayout,
	final_layout: flags::ImageLayout,
}

impl Default for Attachment {
	fn default() -> Attachment {
		Attachment {
			format: flags::Format::UNDEFINED,
			samples: flags::SampleCount::TYPE_1,
			general_ops: Default::default(),
			stencil_ops: Default::default(),
			initial_layout: flags::ImageLayout::UNDEFINED,
			final_layout: flags::ImageLayout::UNDEFINED,
		}
	}
}

impl Attachment {
	pub fn set_format(mut self, format: flags::Format) -> Self {
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

impl utility::VulkanInfo<backend::vk::AttachmentDescription> for Attachment {
	fn to_vk(&self) -> backend::vk::AttachmentDescription {
		backend::vk::AttachmentDescription::builder()
			.format(self.format)
			.samples(self.samples)
			.load_op(self.general_ops.load)
			.store_op(self.general_ops.store)
			.stencil_load_op(self.stencil_ops.load)
			.stencil_store_op(self.stencil_ops.store)
			.initial_layout(self.initial_layout)
			.final_layout(self.final_layout)
			.build()
	}
}
