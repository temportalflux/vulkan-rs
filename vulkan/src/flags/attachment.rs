use super::{LoadOp, StoreOp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttachmentKind {
	Input,
	Color,
	Resolve,
	Preserve,
	DepthStencil,
}

/// The load and store operations that can be performed on an image
/// that is attached to a ['Render Pass'](crate::renderpass::Pass)
/// and its ['Subpasses'](crate::renderpass::Subpass).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct AttachmentOps {
	pub load: LoadOp,
	pub store: StoreOp,
}
