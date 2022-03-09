use super::Attachment;
use crate::flags::{AttachmentKind, ImageLayout};
use std::sync::Arc;

pub struct Reference {
	attachment: Arc<Attachment>,
	kind: AttachmentKind,
	layout: ImageLayout,
}

impl From<&Arc<Attachment>> for Reference {
	fn from(attachment: &Arc<Attachment>) -> Self {
		Self::new(attachment.clone())
	}
}

impl Reference {
	pub fn new(attachment: Arc<Attachment>) -> Self {
		Self {
			attachment,
			kind: AttachmentKind::Color,
			layout: ImageLayout::Undefined,
		}
	}

	pub fn attachment(&self) -> &Arc<Attachment> {
		&self.attachment
	}

	pub fn with_kind(mut self, kind: AttachmentKind) -> Self {
		self.kind = kind;
		self
	}

	pub fn kind(&self) -> AttachmentKind {
		self.kind
	}

	pub fn with_layout(mut self, layout: ImageLayout) -> Self {
		self.layout = layout;
		self
	}

	pub fn layout(&self) -> ImageLayout {
		self.layout
	}
}
