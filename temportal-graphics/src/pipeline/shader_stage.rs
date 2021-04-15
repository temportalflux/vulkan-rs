use crate::{flags::ShaderStageKind, shader, utility, utility::VulkanObject};
use erupt;

pub struct ShaderStage {
	_internal: erupt::vk::PipelineShaderStageCreateInfo,
}

impl ShaderStage {
	pub fn new(module: &shader::Module) -> ShaderStage {
		ShaderStage {
			_internal: erupt::vk::PipelineShaderStageCreateInfo::default(),
		}
		.set_kind(module.kind())
		.set_entry_point(module.entry_point().as_str())
		.set_module(&module)
	}

	pub fn set_kind(mut self, kind: ShaderStageKind) -> Self {
		self._internal.stage = kind as _;
		self
	}

	pub fn set_module(mut self, module: &shader::Module) -> Self {
		self._internal.module = *module.unwrap() as _;
		self
	}

	pub fn set_entry_point(mut self, entry_point: &str) -> Self {
		self._internal.p_name = utility::to_cstr(entry_point) as _;
		self
	}
}

impl Into<erupt::vk::PipelineShaderStageCreateInfo> for ShaderStage {
	fn into(self) -> erupt::vk::PipelineShaderStageCreateInfo {
		self._internal
	}
}
