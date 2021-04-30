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
	entry_point: std::ffi::CString,
	kind: ShaderKind,
	internal: backend::vk::ShaderModule,
	device: Rc<logical::Device>,
}

impl Module {
	pub fn create(device: Rc<logical::Device>, info: shader::Info) -> utility::Result<Module> {
		Ok(Module::create_from_bytes(device, &info.bytes[..])?
			.set_entry_point(info.entry_point)
			.set_kind(info.kind))
	}

	/// Creates a shader module from bytes loaded from a `.spirv` file.
	/// These bytes are created from the engine building a shader asset.
	pub fn create_from_bytes(device: Rc<logical::Device>, bytes: &[u8]) -> utility::Result<Module> {
		use backend::version::DeviceV1_0;

		let decoded_bytes = match backend::util::read_spv(&mut std::io::Cursor::new(bytes)) {
			Ok(bytes) => bytes,
			Err(e) => return Err(utility::Error::General(e)),
		};
		let info = backend::vk::ShaderModuleCreateInfo::builder()
			.code(&decoded_bytes)
			.build();

		let internal =
			utility::as_vulkan_error(unsafe { device.unwrap().create_shader_module(&info, None) })?;
		Ok(Module {
			device,
			internal,
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
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::ShaderModule {
		&mut self.internal
	}
}

impl Drop for Module {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.unwrap()
				.destroy_shader_module(self.internal, None)
		};
	}
}

impl VulkanInfo<backend::vk::PipelineShaderStageCreateInfo> for Module {
	fn to_vk(&self) -> backend::vk::PipelineShaderStageCreateInfo {
		backend::vk::PipelineShaderStageCreateInfo::builder()
			.stage(self.kind.to_vk())
			.module(*self.unwrap())
			.name(&self.entry_point)
			.build()
	}
}
