use crate::backend::vk::ShaderStageFlagBits as ShaderStageKind;
use serde::{Deserialize, Serialize};
use shaderc;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
