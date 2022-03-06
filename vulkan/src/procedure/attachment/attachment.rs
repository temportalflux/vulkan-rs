use crate::{backend, flags};
use crate::{
	flags::{format::Format, AttachmentOps, ImageLayout, SampleCount},
	renderpass::ClearValue,
};

/// Information about an image attached to a ['Render Pass'](crate::renderpass::Pass).
/// Most frequent use is to describe the ['Swapchain'](crate::device::swapchain::khr::Swapchain)
/// images used for each frame that is shown.
#[derive(Debug, Clone, Default)]
pub struct Attachment {
	format: Format,
	sample_count: SampleCount,
	general_ops: AttachmentOps,
	stencil_ops: AttachmentOps,
	initial_layout: ImageLayout,
	final_layout: ImageLayout,
	clear_value: Option<ClearValue>,
}

impl Attachment {
	pub fn with_format(mut self, format: Format) -> Self {
		self.format = format;
		self
	}

	pub fn format(&self) -> Format {
		self.format
	}

	pub fn with_sample_count(mut self, count: SampleCount) -> Self {
		self.sample_count = count;
		self
	}

	pub fn sample_count(&self) -> SampleCount {
		self.sample_count
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

	pub fn with_clear_value(mut self, clear_value: ClearValue) -> Self {
		self.clear_value = Some(clear_value);
		self
	}

	pub fn clear_value(&self) -> Option<&ClearValue> {
		self.clear_value.as_ref()
	}
}

impl Attachment {
	pub fn as_desc(&self) -> backend::vk::AttachmentDescription {
		assert_ne!(self.format, backend::vk::Format::UNDEFINED);
		backend::vk::AttachmentDescription::builder()
			.format(self.format)
			.samples(self.sample_count.into())
			.load_op(self.general_ops.load.into())
			.store_op(self.general_ops.store.into())
			.stencil_load_op(self.stencil_ops.load.into())
			.stencil_store_op(self.stencil_ops.store.into())
			.initial_layout(self.initial_layout.into())
			.final_layout(self.final_layout.into())
			.build()
	}
}
