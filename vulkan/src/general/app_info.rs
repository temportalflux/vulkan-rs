use crate::{backend, utility};

/// Information about the engine and the application using Vulkan.
#[derive(Debug, Clone, Default)]
pub struct AppInfo {
	api_version: u32,

	engine_name: String,
	engine_name_c: std::ffi::CString,
	engine_version: u32,

	app_name: String,
	app_name_c: std::ffi::CString,
	app_version: u32,
}

impl AppInfo {
	/// Creates an application info struct based on the current context.
	pub fn new() -> AppInfo {
		AppInfo {
			api_version: backend::vk::make_version(1, 2, 0),
			engine_name: String::new(),
			engine_name_c: std::ffi::CString::default(),
			engine_version: 0,
			app_name: String::new(),
			app_name_c: std::ffi::CString::default(),
			app_version: 0,
		}
	}

	/// Returns a string-represenation (`major.minor.patch`) of the Vulkan api version.
	pub fn api_version(&self) -> String {
		utility::as_version_string(&self.api_version)
	}

	/// Sets the engine name and version. Use [`utility::make_version`] to create a packed version integer.
	pub fn engine(mut self, name: &str, version: u32) -> AppInfo {
		self.engine_name = String::from(name);
		self.engine_name_c = std::ffi::CString::new(name).unwrap();
		self.engine_version = version;
		self
	}

	pub fn engine_name(&self) -> &str {
		self.engine_name.as_str()
	}

	/// Returns a string-represenation (`major.minor.patch`) of the engine version.
	pub fn engine_version(&self) -> String {
		utility::as_version_string(&self.engine_version)
	}

	/// Sets the application name and version. Use [`utility::make_version`] to create a packed version integer.
	pub fn with_application(mut self, name: &str, version: u32) -> Self {
		self.set_application_info(name, version);
		self
	}

	pub fn set_application_info(&mut self, name: &str, version: u32) {
		self.app_name = String::from(name);
		self.app_name_c = std::ffi::CString::new(name).unwrap();
		self.app_version = version;
	}

	pub fn app_name(&self) -> &String {
		&self.app_name
	}

	/// Returns a string-represenation (`major.minor.patch`) of the application version.
	pub fn app_version(&self) -> String {
		utility::as_version_string(&self.app_version)
	}

	/// Returns a stringified description of the info as:
	/// "Vulkan(v`#.#.#`) for `engine name`(v`#.#.#`) running `app name`(v`#.#.#`)"
	pub fn description(&self) -> String {
		format!(
			"Vulkan(v{}) for {}(v{}) running {}(v{})",
			self.api_version(),
			self.engine_name,
			self.engine_version(),
			self.app_name,
			self.app_version()
		)
	}

	pub(crate) fn as_vk(&self) -> backend::vk::ApplicationInfo {
		backend::vk::ApplicationInfo::builder()
			.api_version(self.api_version)
			.engine_name(&self.engine_name_c)
			.engine_version(self.engine_version)
			.application_name(&self.app_name_c)
			.application_version(self.app_version)
			.build()
	}
}
