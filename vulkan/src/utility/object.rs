use crate::backend;

pub trait HandledObject {
	fn kind(&self) -> backend::vk::ObjectType;
	fn handle(&self) -> u64;

	fn create_name<T>(&self, name: T) -> ObjectName
	where
		T: Into<String>,
	{
		ObjectName::from(name)
			.with_kind(self.kind())
			.with_raw_handle(self.handle())
	}
}

pub trait NamedObject {
	fn name(&self) -> &Option<String>;
}

pub struct ObjectName {
	name: String,
	name_raw: std::ffi::CString,
	kind: backend::vk::ObjectType,
	handle: u64,
}

impl<T> From<T> for ObjectName
where
	T: Into<String>,
{
	fn from(s: T) -> Self {
		let name: String = s.into();
		let name_raw = std::ffi::CString::new(name.as_bytes()).unwrap();
		ObjectName {
			name,
			name_raw,
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
