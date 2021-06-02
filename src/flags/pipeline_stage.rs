use crate::backend::vk::PipelineStageFlags as VkEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, enumset::EnumSetType)]
pub enum PipelineStage {
	/// Before subsequent commands are processed
	TopOfPipe,
	/// Draw/DispatchIndirect command fetch
	DrawIndirect,
	/// Vertex/index fetch
	VertexInput,
	/// Vertex shading
	VertexShader,
	/// Tessellation control shading
	TessellationControlShader,
	/// Tessellation evaluation shading
	TessellationEvaluationShader,
	/// Geometry shading
	GeometryShader,
	/// Fragment shading
	FragmentShader,
	/// Early fragment (depth and stencil) tests
	EarlyFragmentTests,
	/// Late fragment (depth and stencil) tests
	LateFragmentTests,
	/// Color attachment writes
	ColorAttachmentOutput,
	/// Compute shading
	ComputerShader,
	/// Transfer/copy operations
	Transfer,
	/// After previous commands have completed
	BottomOfPipe,
	/// Indicates host (CPU) is a source/sink of the dependency
	Host,
	/// All stages of the graphics pipeline
	AllGraphics,
	/// All stages supported on the queue
	AllCommands,
}

impl Into<VkEnum> for PipelineStage {
	fn into(self) -> VkEnum {
		match self {
			Self::TopOfPipe => VkEnum::TOP_OF_PIPE,
			Self::DrawIndirect => VkEnum::DRAW_INDIRECT,
			Self::VertexInput => VkEnum::VERTEX_INPUT,
			Self::VertexShader => VkEnum::VERTEX_SHADER,
			Self::TessellationControlShader => VkEnum::TESSELLATION_CONTROL_SHADER,
			Self::TessellationEvaluationShader => VkEnum::TESSELLATION_EVALUATION_SHADER,
			Self::GeometryShader => VkEnum::GEOMETRY_SHADER,
			Self::FragmentShader => VkEnum::FRAGMENT_SHADER,
			Self::EarlyFragmentTests => VkEnum::EARLY_FRAGMENT_TESTS,
			Self::LateFragmentTests => VkEnum::LATE_FRAGMENT_TESTS,
			Self::ColorAttachmentOutput => VkEnum::COLOR_ATTACHMENT_OUTPUT,
			Self::ComputerShader => VkEnum::COMPUTE_SHADER,
			Self::Transfer => VkEnum::TRANSFER,
			Self::BottomOfPipe => VkEnum::BOTTOM_OF_PIPE,
			Self::Host => VkEnum::HOST,
			Self::AllGraphics => VkEnum::ALL_GRAPHICS,
			Self::AllCommands => VkEnum::ALL_COMMANDS,
		}
	}
}

impl PipelineStage {
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
