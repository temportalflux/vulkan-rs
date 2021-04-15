use erupt;
use crate::{utility::VulkanObject, device::logical};

pub struct Shader {
	_internal: erupt::vk::ShaderModule,
}

impl Shader {
	/// Creates a shader module from bytes loaded from a `.spirv` file.
	/// These bytes are created from the engine building a shader asset.
	pub fn create(device: &logical::Device, bytes: &[u8]) -> Result<Shader, Box<dyn std::error::Error>> {
		let decoded_bytes = erupt::utils::decode_spv(bytes)?;
		let info = erupt::vk::ShaderModuleCreateInfoBuilder::new().code(&decoded_bytes).build();
		Ok(Shader {
			_internal: device.create_shader_module(info),
		})
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::ShaderModule`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::ShaderModule> for Shader {
	fn unwrap(&self) -> &erupt::vk::ShaderModule {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::ShaderModule {
		&mut self._internal
	}
}
