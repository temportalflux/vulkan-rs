use crate::{
	backend,
	flags::{blend, ColorComponent},
};
use enumset::EnumSet;

/// Struct containing information about how a [`Pipeline`](crate::pipeline::Pipeline)
/// blends the color of its [`attachments`](crate::renderpass::Attachment).
pub struct ColorBlend {
	pub attachments: Vec<backend::vk::PipelineColorBlendAttachmentState>,
}

impl Default for ColorBlend {
	fn default() -> Self {
		Self {
			attachments: Vec::new(),
		}
	}
}

/// The properties of a specific attachment in the pipeline and its color blending.
#[derive(Clone, Copy)]
pub struct Attachment {
	pub color_flags: EnumSet<ColorComponent>,
	pub blend: Option<Blend>,
}

impl Default for Attachment {
	fn default() -> Self {
		Self {
			color_flags: EnumSet::all(),
			blend: Some(Blend::alpha_blend()),
		}
	}
}

#[derive(Clone, Copy)]
pub struct Blend {
	pub color: blend::Expression,
	pub alpha: blend::Expression,
}

impl Blend {
	pub fn alpha_blend() -> Self {
		use blend::{Constant::*, Factor::*, Source::*};
		Self {
			color: SrcAlpha * New + (One - SrcAlpha) * Old,
			alpha: One * New + Zero * Old,
		}
	}
}

impl ColorBlend {
	pub fn add_attachment(mut self, attachment: Attachment) -> Self {
		self.attachments.push(
			backend::vk::PipelineColorBlendAttachmentState::builder()
				.color_write_mask(ColorComponent::fold(&attachment.color_flags))
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
