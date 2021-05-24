use crate::{backend, device::logical, flags::ShaderKind, shader, utility};
use std::sync;

pub struct Info {
	pub kind: ShaderKind,
	pub entry_point: String,
	pub bytes: Vec<u8>,
}

pub struct Module {
	entry_point: std::ffi::CString,
	kind: ShaderKind,
	internal: backend::vk::ShaderModule,
	device: sync::Arc<logical::Device>,
}

impl Module {
	pub fn create(
		device: sync::Arc<logical::Device>,
		info: shader::Info,
	) -> utility::Result<Module> {
		Ok(Module::create_from_bytes(device, &info.bytes[..])?
			.set_entry_point(info.entry_point)
			.set_kind(info.kind))
	}

	/// Creates a shader module from bytes loaded from a `.spirv` file.
	/// These bytes are created from the engine building a shader asset.
	pub fn create_from_bytes(
		device: sync::Arc<logical::Device>,
		bytes: &[u8],
	) -> utility::Result<Module> {
		use backend::version::DeviceV1_0;

		let decoded_bytes = match backend::util::read_spv(&mut std::io::Cursor::new(bytes)) {
			Ok(bytes) => bytes,
			Err(e) => return Err(utility::Error::General(e)),
		};
		let info = backend::vk::ShaderModuleCreateInfo::builder()
			.code(&decoded_bytes)
			.build();

		let internal = unsafe { device.create_shader_module(&info, None) }?;
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

	pub fn kind(&self) -> ShaderKind {
		self.kind
	}

	pub fn entry_point(&self) -> &std::ffi::CStr {
		&self.entry_point
	}
}

impl std::ops::Deref for Module {
	type Target = backend::vk::ShaderModule;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Module {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.destroy_shader_module(self.internal, None) };
	}
}
