use crate::backend::vk::ShaderStageFlags as ShaderStageKind;
use serde::{Deserialize, Serialize};
use shaderc;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderKind {
	Vertex,
	Fragment,
	Geometry,
	Compute,
	TessControl,
	TessEval,
	Task,
	Mesh,
	Raygen,
	AnyHit,
	ClosedHit,
	Miss,
	Intersection,
	Callable,
}

impl Into<ShaderStageKind> for ShaderKind {
	fn into(self) -> ShaderStageKind {
		self.to_vk()
	}
}

impl ShaderKind {
	pub fn to_vk(&self) -> ShaderStageKind {
		match self {
			ShaderKind::Vertex => ShaderStageKind::VERTEX,
			ShaderKind::Fragment => ShaderStageKind::FRAGMENT,
			ShaderKind::Geometry => ShaderStageKind::GEOMETRY,
			ShaderKind::Compute => ShaderStageKind::COMPUTE,
			ShaderKind::TessControl => ShaderStageKind::TESSELLATION_CONTROL,
			ShaderKind::TessEval => ShaderStageKind::TESSELLATION_EVALUATION,
			ShaderKind::Task => ShaderStageKind::TASK_NV,
			ShaderKind::Mesh => ShaderStageKind::MESH_NV,
			ShaderKind::Raygen => ShaderStageKind::RAYGEN_KHR,
			ShaderKind::AnyHit => ShaderStageKind::ANY_HIT_KHR,
			ShaderKind::ClosedHit => ShaderStageKind::CLOSEST_HIT_KHR,
			ShaderKind::Miss => ShaderStageKind::MISS_KHR,
			ShaderKind::Intersection => ShaderStageKind::INTERSECTION_KHR,
			ShaderKind::Callable => ShaderStageKind::CALLABLE_KHR,
		}
	}
	pub fn to_shaderc(&self) -> shaderc::ShaderKind {
		match self {
			ShaderKind::Vertex => shaderc::ShaderKind::Vertex,
			ShaderKind::Fragment => shaderc::ShaderKind::Fragment,
			ShaderKind::Geometry => shaderc::ShaderKind::Geometry,
			ShaderKind::Compute => shaderc::ShaderKind::Compute,
			ShaderKind::TessControl => shaderc::ShaderKind::TessControl,
			ShaderKind::TessEval => shaderc::ShaderKind::TessEvaluation,
			_ => panic!("No such shader kind"),
		}
	}
	fn _to_string(&self) -> String {
		match self {
			ShaderKind::Vertex => "vertex".to_string(),
			ShaderKind::Fragment => "fragment".to_string(),
			ShaderKind::Geometry => "geometry".to_string(),
			ShaderKind::Compute => "compute".to_string(),
			ShaderKind::TessControl => "tess-control".to_string(),
			ShaderKind::TessEval => "tess-eval".to_string(),
			_ => String::new(),
		}
	}
	fn _from_string(s: &str) -> Option<ShaderKind> {
		match s {
			"vertex" => Some(ShaderKind::Vertex),
			"fragment" => Some(ShaderKind::Fragment),
			"geometry" => Some(ShaderKind::Geometry),
			"compute" => Some(ShaderKind::Compute),
			"tess-control" => Some(ShaderKind::TessControl),
			"tess-eval" => Some(ShaderKind::TessEval),
			_ => None,
		}
	}
}

impl Into<String> for ShaderKind {
	fn into(self) -> String {
		match self {
			Self::Vertex => "Vertex",
			Self::Fragment => "Fragment",
			Self::Geometry => "Geometry",
			Self::Compute => "Compute",
			Self::TessControl => "TessControl",
			Self::TessEval => "TessEval",
			Self::Task => "Task",
			Self::Mesh => "Mesh",
			Self::Raygen => "Raygen",
			Self::AnyHit => "AnyHit",
			Self::ClosedHit => "ClosedHit",
			Self::Miss => "Miss",
			Self::Intersection => "Intersection",
			Self::Callable => "Callable",
		}
		.to_string()
	}
}
