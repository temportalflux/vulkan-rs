use crate::{flags, utility::VulkanInfo};
use erupt;

/// A rendering phase of a given [`Render Pass`](crate::renderpass::Pass),
/// which may correlate to the rendering of one or more [`Pipelines`](crate::pipeline::Pipeline).
pub struct Subpass {
	bind_point: flags::PipelineBindPoint,
	attachment_refs: Vec<erupt::vk::AttachmentReference>,
}

impl Default for Subpass {
	fn default() -> Subpass {
		Subpass {
			bind_point: flags::PipelineBindPoint::GRAPHICS,
			attachment_refs: Vec::new(),
		}
	}
}

impl Subpass {
	pub fn add_attachment_ref(
		mut self,
		attachment_index: usize,
		layout: flags::ImageLayout,
	) -> Self {
		self.attachment_refs.push(
			erupt::vk::AttachmentReferenceBuilder::new()
				.attachment(attachment_index as u32)
				.layout(layout)
				.build(),
		);
		self
	}
}

impl VulkanInfo<erupt::vk::SubpassDescription> for Subpass {
	fn to_vk(&self) -> erupt::vk::SubpassDescription {
		erupt::vk::SubpassDescriptionBuilder::new()
			.pipeline_bind_point(self.bind_point)
			.color_attachments(&crate::into_builders!(self.attachment_refs))
			.build()
	}
}
