use crate::flags::{self, AttachmentKind};

/// A rendering phase of a given [`Render Pass`](crate::renderpass::Pass),
/// which may correlate to the rendering of one or more [`Pipelines`](crate::pipeline::Pipeline).
#[derive(Debug, Clone)]
pub struct Subpass {
	id: String,
	bind_point: flags::PipelineBindPoint,
	attachments: SubpassAttachments,
}

#[derive(Debug, Clone)]
pub(crate) struct SubpassAttachments {
	pub input: Vec<(String, flags::ImageLayout)>,
	pub color: Vec<(String, flags::ImageLayout)>,
	pub resolve: Vec<(String, flags::ImageLayout)>,
	pub preserve: Vec<String>,
	pub depth_stencil: Option<(String, flags::ImageLayout)>,
}

impl Subpass {
	pub fn new(id: String) -> Self {
		Self {
			id,
			bind_point: flags::PipelineBindPoint::GRAPHICS,
			attachments: SubpassAttachments {
				color: Vec::new(),
				input: Vec::new(),
				resolve: Vec::new(),
				preserve: Vec::new(),
				depth_stencil: None,
			},
		}
	}

	pub fn add_attachment(
		mut self,
		id: String,
		kind: AttachmentKind,
		layout: Option<flags::ImageLayout>,
	) -> Self {
		match kind {
			AttachmentKind::Input => self.attachments.input.push((id, layout.unwrap())),
			AttachmentKind::Color => self.attachments.color.push((id, layout.unwrap())),
			AttachmentKind::Resolve => self.attachments.resolve.push((id, layout.unwrap())),
			AttachmentKind::Preserve => self.attachments.preserve.push(id),
			AttachmentKind::DepthStencil => {
				self.attachments.depth_stencil = Some((id, layout.unwrap()))
			}
		}
		self
	}

	pub fn add_input_attachment(self, attachment_id: String, layout: flags::ImageLayout) -> Self {
		self.add_attachment(attachment_id, AttachmentKind::Input, Some(layout))
	}

	pub fn add_color_attachment(self, attachment_id: String, layout: flags::ImageLayout) -> Self {
		self.add_attachment(attachment_id, AttachmentKind::Color, Some(layout))
	}

	pub fn with_depth_stencil_attachment(
		self,
		attachment_id: String,
		layout: flags::ImageLayout,
	) -> Self {
		self.add_attachment(attachment_id, AttachmentKind::DepthStencil, Some(layout))
	}

	pub fn id(&self) -> &String {
		&self.id
	}

	pub fn bind_point(&self) -> flags::PipelineBindPoint {
		self.bind_point
	}

	pub(crate) fn attachments(&self) -> &SubpassAttachments {
		&self.attachments
	}
}
