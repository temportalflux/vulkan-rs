use crate::{
	backend,
	flags::{blend, ColorComponent},
};

/// Struct containing information about how a [`Pipeline`](crate::pipeline::Pipeline)
/// blends the color of its [`attachments`](crate::renderpass::Attachment).
pub struct ColorBlendState {
	pub attachments: Vec<backend::vk::PipelineColorBlendAttachmentState>,
}

impl Default for ColorBlendState {
	fn default() -> ColorBlendState {
		ColorBlendState {
			attachments: Vec::new(),
		}
	}
}

/// The properties of a specific attachment in the pipeline and its color blending.
#[derive(Clone, Copy)]
pub struct ColorBlendAttachment {
	pub color_flags: ColorComponent,
	pub blend: Option<Blend>,
}

#[derive(Clone, Copy)]
pub struct Blend {
	pub color: blend::Expression,
	pub alpha: blend::Expression,
}

impl ColorBlendState {
	pub fn add_attachment(mut self, attachment: ColorBlendAttachment) -> Self {
		self.attachments.push(
			backend::vk::PipelineColorBlendAttachmentState::builder()
				.color_write_mask(attachment.color_flags)
				.blend_enable(attachment.blend.is_some())
				.src_color_blend_factor(
					attachment
						.blend
						.map_or(backend::vk::BlendFactor::ONE, |blend| blend.color.src),
				)
				.color_blend_op(
					attachment
						.blend
						.map_or(backend::vk::BlendOp::ADD, |blend| blend.color.op),
				)
				.dst_color_blend_factor(
					attachment
						.blend
						.map_or(backend::vk::BlendFactor::ZERO, |blend| blend.color.dst),
				)
				.src_alpha_blend_factor(
					attachment
						.blend
						.map_or(backend::vk::BlendFactor::ONE, |blend| blend.alpha.src),
				)
				.color_blend_op(
					attachment
						.blend
						.map_or(backend::vk::BlendOp::ADD, |blend| blend.alpha.op),
				)
				.dst_alpha_blend_factor(
					attachment
						.blend
						.map_or(backend::vk::BlendFactor::ZERO, |blend| blend.alpha.dst),
				)
				.build(),
		);
		self
	}
}
