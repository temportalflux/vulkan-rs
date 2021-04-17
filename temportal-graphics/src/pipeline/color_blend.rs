use crate::flags::ColorComponent;

/// Struct containing information about how a [`Pipeline`](crate::pipeline::Pipeline)
/// blends the color of its [`attachments`](crate::renderpass::Attachment).
pub struct ColorBlendState {
	pub attachments: Vec<erupt::vk::PipelineColorBlendAttachmentState>,
}

impl Default for ColorBlendState {
	fn default() -> ColorBlendState {
		ColorBlendState {
			attachments: Vec::new(),
		}
	}
}

/// The properties of a specific attachment in the pipeline and its color blending.
pub struct ColorBlendAttachment {
	pub color_flags: ColorComponent,
}

impl ColorBlendState {
	pub fn add_attachment(mut self, attachment: ColorBlendAttachment) -> Self {
		self.attachments.push(
			erupt::vk::PipelineColorBlendAttachmentStateBuilder::new()
				.color_write_mask(attachment.color_flags)
				.blend_enable(false)
				.build(),
		);
		self
	}
}
