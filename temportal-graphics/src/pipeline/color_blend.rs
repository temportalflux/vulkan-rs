use crate::flags::ColorComponent;

pub struct ColorBlendState {
	pub attachments: Vec<erupt::vk::PipelineColorBlendAttachmentState>,
}

pub struct ColorBlendAttachment {
	pub color_flags: ColorComponent,
}

impl ColorBlendState {
	pub fn new() -> ColorBlendState {
		ColorBlendState {
			attachments: Vec::new(),
		}
	}

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
