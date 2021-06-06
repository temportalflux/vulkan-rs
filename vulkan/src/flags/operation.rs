use crate::backend::vk::{AttachmentLoadOp, AttachmentStoreOp};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadOp {
	Load,
	Clear,
	DontCare,
}

impl Default for LoadOp {
	fn default() -> Self {
		Self::DontCare
	}
}

impl Into<AttachmentLoadOp> for LoadOp {
	fn into(self) -> AttachmentLoadOp {
		match self {
			Self::Load => AttachmentLoadOp::LOAD,
			Self::Clear => AttachmentLoadOp::CLEAR,
			Self::DontCare => AttachmentLoadOp::DONT_CARE,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreOp {
	Store,
	DontCare,
}

impl Default for StoreOp {
	fn default() -> Self {
		Self::DontCare
	}
}

impl Into<AttachmentStoreOp> for StoreOp {
	fn into(self) -> AttachmentStoreOp {
		match self {
			Self::Store => AttachmentStoreOp::STORE,
			Self::DontCare => AttachmentStoreOp::DONT_CARE,
		}
	}
}
