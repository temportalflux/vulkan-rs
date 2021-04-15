use crate::into_builders;
use crate::{flags::ColorComponent, utility::VulkanInfo};

pub struct ColorBlendState {
	attachments: Vec<erupt::vk::PipelineColorBlendAttachmentState>,
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

impl VulkanInfo<erupt::vk::PipelineColorBlendStateCreateInfo> for ColorBlendState {
	fn to_vk(&self) -> erupt::vk::PipelineColorBlendStateCreateInfo {
		erupt::vk::PipelineColorBlendStateCreateInfoBuilder::new()
			.logic_op_enable(false)
			.attachments(into_builders!(self.attachments))
			.build()
	}
}
