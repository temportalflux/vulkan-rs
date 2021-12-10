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

	pub fn all_serialized() -> Vec<String> {
		enumset::EnumSet::<Self>::all()
			.iter()
			.map(|flag| flag.to_string())
			.collect()
	}
}

impl ToString for PipelineStage {
	fn to_string(&self) -> String {
		match self {
			Self::TopOfPipe => "TopOfPipe",
			Self::DrawIndirect => "DrawIndirect",
			Self::VertexInput => "VertexInput",
			Self::VertexShader => "VertexShader",
			Self::TessellationControlShader => "TessellationControlShader",
			Self::TessellationEvaluationShader => "TessellationEvaluationShader",
			Self::GeometryShader => "GeometryShader",
			Self::FragmentShader => "FragmentShader",
			Self::EarlyFragmentTests => "EarlyFragmentTests",
			Self::LateFragmentTests => "LateFragmentTests",
			Self::ColorAttachmentOutput => "ColorAttachmentOutput",
			Self::ComputerShader => "ComputerShader",
			Self::Transfer => "Transfer",
			Self::BottomOfPipe => "BottomOfPipe",
			Self::Host => "Host",
			Self::AllGraphics => "AllGraphics",
			Self::AllCommands => "AllCommands",
		}
		.to_owned()
	}
}

impl std::convert::TryFrom<&str> for PipelineStage {
	type Error = ();
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"TopOfPipe" => Ok(Self::TopOfPipe),
			"DrawIndirect" => Ok(Self::DrawIndirect),
			"VertexInput" => Ok(Self::VertexInput),
			"VertexShader" => Ok(Self::VertexShader),
			"TessellationControlShader" => Ok(Self::TessellationControlShader),
			"TessellationEvaluationShader" => Ok(Self::TessellationEvaluationShader),
			"GeometryShader" => Ok(Self::GeometryShader),
			"FragmentShader" => Ok(Self::FragmentShader),
			"EarlyFragmentTests" => Ok(Self::EarlyFragmentTests),
			"LateFragmentTests" => Ok(Self::LateFragmentTests),
			"ColorAttachmentOutput" => Ok(Self::ColorAttachmentOutput),
			"ComputerShader" => Ok(Self::ComputerShader),
			"Transfer" => Ok(Self::Transfer),
			"BottomOfPipe" => Ok(Self::BottomOfPipe),
			"Host" => Ok(Self::Host),
			"AllGraphics" => Ok(Self::AllGraphics),
			"AllCommands" => Ok(Self::AllCommands),
			_ => Err(()),
		}
	}
}
