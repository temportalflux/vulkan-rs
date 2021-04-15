use crate::{device::logical, flags::ShaderStageKind, shader, utility::VulkanObject};
use erupt;

pub struct Module {
	_internal: erupt::vk::ShaderModule,
	entry_point: String,
	kind: ShaderStageKind,
}

impl Module {
	pub fn create(
		device: &logical::Device,
		info: shader::Info,
	) -> Result<Module, Box<dyn std::error::Error>> {
		Ok(Module::create_from_bytes(device, &info.bytes[..])?
			.set_entry_point(info.entry_point)
			.set_kind(info.kind))
	}

	/// Creates a shader module from bytes loaded from a `.spirv` file.
	/// These bytes are created from the engine building a shader asset.
	pub fn create_from_bytes(
		device: &logical::Device,
		bytes: &[u8],
	) -> Result<Module, Box<dyn std::error::Error>> {
		let decoded_bytes = erupt::utils::decode_spv(bytes)?;
		let info = erupt::vk::ShaderModuleCreateInfoBuilder::new()
			.code(&decoded_bytes)
			.build();
		Ok(Module {
			_internal: device.create_shader_module(info),
			entry_point: String::default(),
			kind: ShaderStageKind::VERTEX,
		})
	}

	pub fn set_entry_point(mut self, entry_point: String) -> Self {
		self.entry_point = entry_point;
		self
	}

	pub fn entry_point(&self) -> String {
		self.entry_point.clone()
	}

	pub fn set_kind(mut self, kind: ShaderStageKind) -> Self {
		self.kind = kind;
		self
	}

	pub fn kind(&self) -> ShaderStageKind {
		self.kind
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::ShaderModule`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::ShaderModule> for Module {
	fn unwrap(&self) -> &erupt::vk::ShaderModule {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::ShaderModule {
		&mut self._internal
	}
}
