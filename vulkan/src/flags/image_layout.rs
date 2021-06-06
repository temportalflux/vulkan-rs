use crate::backend::vk::ImageLayout as VkEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageLayout {
	/// Implicit layout an image is when its contents are undefined due to various reasons (e.g. right after creation)
	Undefined,
	/// General layout when image can be used for any kind of access
	General,
	/// Optimal layout when image is only used for color attachment read/write
	ColorAttachmentOptimal,
	/// Optimal layout when image is only used for depth/stencil attachment read/write
	DepthStencilAttachmentOptimal,
	/// Optimal layout when image is used for read only depth/stencil attachment and shader access
	DepthStencilReadOnlyOptimal,
	/// Optimal layout when image is used for read only shader access
	ShaderReadOnlyOptimal,
	/// Optimal layout when image is used only as source of transfer operations
	TransferSrcOptimal,
	/// Optimal layout when image is used only as destination of transfer operations
	TransferDstOptimal,
	/// Initial layout used when the data is populated by the CPU
	Preinitialized,

	// KHR
	PresentSrc,
}

impl Default for ImageLayout {
	fn default() -> Self {
		Self::Undefined
	}
}

impl Into<VkEnum> for ImageLayout {
	fn into(self) -> VkEnum {
		match self {
			Self::Undefined => VkEnum::UNDEFINED,
			Self::General => VkEnum::GENERAL,
			Self::ColorAttachmentOptimal => VkEnum::COLOR_ATTACHMENT_OPTIMAL,
			Self::DepthStencilAttachmentOptimal => VkEnum::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
			Self::DepthStencilReadOnlyOptimal => VkEnum::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
			Self::ShaderReadOnlyOptimal => VkEnum::SHADER_READ_ONLY_OPTIMAL,
			Self::TransferSrcOptimal => VkEnum::TRANSFER_SRC_OPTIMAL,
			Self::TransferDstOptimal => VkEnum::TRANSFER_DST_OPTIMAL,
			Self::Preinitialized => VkEnum::PREINITIALIZED,
			Self::PresentSrc => VkEnum::PRESENT_SRC_KHR,
		}
	}
}
