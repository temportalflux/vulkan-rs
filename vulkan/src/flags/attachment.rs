use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttachmentKind {
	Input,
	Color,
	Resolve,
	Preserve,
	DepthStencil,
}
