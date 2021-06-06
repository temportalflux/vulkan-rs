use crate::flags;

/// A rendering phase of a given [`Render Pass`](crate::renderpass::Pass),
/// which may correlate to the rendering of one or more [`Pipelines`](crate::pipeline::Pipeline).
#[derive(Debug)]
pub struct Subpass {
	id: String,
	bind_point: flags::PipelineBindPoint,
	attachments: SubpassAttachments,
}

#[derive(Debug, Clone)]
pub(crate) struct SubpassAttachments {
	pub input: Vec<(String, flags::ImageLayout)>,
	pub color: Vec<(String, flags::ImageLayout)>,
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
				depth_stencil: None,
			},
		}
	}

	pub fn add_input_attachment(
		mut self,
		attachment_id: String,
		layout: flags::ImageLayout,
	) -> Self {
		self.attachments.input.push((attachment_id, layout));
		self
	}

	pub fn add_color_attachment(
		mut self,
		attachment_id: String,
		layout: flags::ImageLayout,
	) -> Self {
		self.attachments.color.push((attachment_id, layout));
		self
	}

	pub fn with_depth_stencil_attachment(
		mut self,
		attachment_id: String,
		layout: flags::ImageLayout,
	) -> Self {
		self.attachments.depth_stencil = Some((attachment_id, layout));
		self
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
