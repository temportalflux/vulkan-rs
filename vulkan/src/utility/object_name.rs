use crate::backend;

pub trait NamableObject {
	fn kind(&self) -> backend::vk::ObjectType;
	fn handle(&self) -> u64;

	fn create_name(&self, name: &str) -> ObjectName {
		ObjectName::from(name)
			.with_kind(self.kind())
			.with_raw_handle(self.handle())
	}
}

pub struct ObjectName {
	name: String,
	name_raw: std::ffi::CString,
	kind: backend::vk::ObjectType,
	handle: u64,
}

impl From<&str> for ObjectName {
	fn from(s: &str) -> Self {
		ObjectName {
			name: s.to_string(),
			name_raw: std::ffi::CString::new(s.as_bytes()).unwrap(),
			kind: Default::default(),
			handle: Default::default(),
		}
	}
}

impl std::fmt::Display for ObjectName {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl ObjectName {
	pub fn with_kind(mut self, kind: backend::vk::ObjectType) -> Self {
		self.kind = kind;
		self
	}

	pub fn with_raw_handle(mut self, handle: u64) -> Self {
		self.handle = handle;
		self
	}

	pub fn with_handle<T>(self, handle: T) -> Self
	where
		T: backend::vk::Handle,
	{
		self.with_raw_handle(handle.as_raw())
	}

	pub fn as_vk(&self) -> backend::vk::DebugUtilsObjectNameInfoEXT {
		backend::vk::DebugUtilsObjectNameInfoEXT::builder()
			.object_type(self.kind)
			.object_handle(self.handle)
			.object_name(self.name_raw.as_c_str())
			.build()
	}
}
