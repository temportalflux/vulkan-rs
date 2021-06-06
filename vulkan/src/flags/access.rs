use crate::backend::vk::AccessFlags as VkEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, enumset::EnumSetType)]
pub enum Access {
	/// Controls coherency of indirect command reads
	IndirectCommandRead,
	/// Controls coherency of index reads
	IndexRead,
	/// Controls coherency of vertex attribute reads
	VertexAttributeRead,
	/// Controls coherency of uniform buffer reads
	UniformRead,
	/// Controls coherency of input attachment reads
	InputAttachmentRead,
	/// Controls coherency of shader reads
	ShaderRead,
	/// Controls coherency of shader writes
	ShaderWrite,
	/// Controls coherency of color attachment reads
	ColorAttachmentRead,
	/// Controls coherency of color attachment writes
	ColorAttachmentWrite,
	/// Controls coherency of depth/stencil attachment reads
	DepthStencilAttachmentRead,
	/// Controls coherency of depth/stencil attachment writes
	DepthStencilAttachmentWrite,
	/// Controls coherency of transfer reads
	TransferRead,
	/// Controls coherency of transfer writes
	TransferWrite,
	/// Controls coherency of host reads
	HostRead,
	/// Controls coherency of host writes
	HostWrite,
	/// Controls coherency of memory reads
	MemoryRead,
	/// Controls coherency of memory writes
	MemoryWrite,
}

impl Into<VkEnum> for Access {
	fn into(self) -> VkEnum {
		match self {
			Self::IndirectCommandRead => VkEnum::INDIRECT_COMMAND_READ,
			Self::IndexRead => VkEnum::INDEX_READ,
			Self::VertexAttributeRead => VkEnum::VERTEX_ATTRIBUTE_READ,
			Self::UniformRead => VkEnum::UNIFORM_READ,
			Self::InputAttachmentRead => VkEnum::INPUT_ATTACHMENT_READ,
			Self::ShaderRead => VkEnum::SHADER_READ,
			Self::ShaderWrite => VkEnum::SHADER_WRITE,
			Self::ColorAttachmentRead => VkEnum::COLOR_ATTACHMENT_READ,
			Self::ColorAttachmentWrite => VkEnum::COLOR_ATTACHMENT_WRITE,
			Self::DepthStencilAttachmentRead => VkEnum::DEPTH_STENCIL_ATTACHMENT_READ,
			Self::DepthStencilAttachmentWrite => VkEnum::DEPTH_STENCIL_ATTACHMENT_WRITE,
			Self::TransferRead => VkEnum::TRANSFER_READ,
			Self::TransferWrite => VkEnum::TRANSFER_WRITE,
			Self::HostRead => VkEnum::HOST_READ,
			Self::HostWrite => VkEnum::HOST_WRITE,
			Self::MemoryRead => VkEnum::MEMORY_READ,
			Self::MemoryWrite => VkEnum::MEMORY_WRITE,
		}
	}
}

impl Access {
	pub fn vecset(vec: &Vec<Self>) -> enumset::EnumSet<Self> {
		vec.iter()
			.fold(enumset::EnumSet::empty(), |mut set, value| {
				set.insert(*value);
				set
			})
	}
	pub fn fold(set: &enumset::EnumSet<Self>) -> VkEnum {
		set.iter()
			.fold(VkEnum::empty(), |vk, value| vk | value.into())
	}
}
