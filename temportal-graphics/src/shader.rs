use crate::{
	backend,
	device::logical,
	flags::ShaderKind,
	shader,
	utility::{self, VulkanInfo, VulkanObject},
};
use std::rc::Rc;

pub struct Info {
	pub kind: ShaderKind,
	pub entry_point: String,
	pub bytes: Vec<u8>,
}

pub struct Module {
	_device: Rc<logical::Device>,
	_internal: backend::vk::ShaderModule,
	entry_point: std::ffi::CString,
	kind: ShaderKind,
}

impl Module {
	pub fn create(device: Rc<logical::Device>, info: shader::Info) -> utility::Result<Module> {
		Ok(Module::create_from_bytes(device, &info.bytes[..])?
			.set_entry_point(info.entry_point)
			.set_kind(info.kind))
	}

	/// Creates a shader module from bytes loaded from a `.spirv` file.
	/// These bytes are created from the engine building a shader asset.
	pub fn create_from_bytes(
		_device: Rc<logical::Device>,
		bytes: &[u8],
	) -> utility::Result<Module> {
		let decoded_bytes = match backend::utils::decode_spv(bytes) {
			Ok(bytes) => bytes,
			Err(e) => return Err(utility::Error::General(e)),
		};
		let info = backend::vk::ShaderModuleCreateInfoBuilder::new()
			.code(&decoded_bytes)
			.build();
		let _internal = _device.create_shader_module(info)?;
		Ok(Module {
			_device,
			_internal,
			entry_point: std::ffi::CString::default(),
			kind: ShaderKind::Vertex,
		})
	}

	pub fn set_entry_point(mut self, entry_point: String) -> Self {
		self.entry_point = std::ffi::CString::new(entry_point).unwrap();
		self
	}

	pub fn set_kind(mut self, kind: ShaderKind) -> Self {
		self.kind = kind;
		self
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::ShaderModule`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::ShaderModule> for Module {
	fn unwrap(&self) -> &backend::vk::ShaderModule {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::ShaderModule {
		&mut self._internal
	}
}

impl Drop for Module {
	fn drop(&mut self) {
		self._device.destroy_shader_module(self._internal);
	}
}

impl VulkanInfo<backend::vk::PipelineShaderStageCreateInfo> for Module {
	fn to_vk(&self) -> backend::vk::PipelineShaderStageCreateInfo {
		backend::vk::PipelineShaderStageCreateInfoBuilder::new()
			.stage(self.kind.to_vk())
			.module(*self.unwrap())
			.name(&self.entry_point)
			.build()
	}
}
