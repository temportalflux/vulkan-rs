use crate::{backend, flags, utility::VulkanInfo};

/// A rendering phase of a given [`Render Pass`](crate::renderpass::Pass),
/// which may correlate to the rendering of one or more [`Pipelines`](crate::pipeline::Pipeline).
pub struct Subpass {
	bind_point: flags::PipelineBindPoint,
	attachment_refs: Vec<backend::vk::AttachmentReference>,
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
			backend::vk::AttachmentReference::builder()
				.attachment(attachment_index as u32)
				.layout(layout)
				.build(),
		);
		self
	}
}

impl VulkanInfo<backend::vk::SubpassDescription> for Subpass {
	fn to_vk(&self) -> backend::vk::SubpassDescription {
		backend::vk::SubpassDescription::builder()
			.pipeline_bind_point(self.bind_point)
			.color_attachments(&self.attachment_refs)
			.build()
	}
}
