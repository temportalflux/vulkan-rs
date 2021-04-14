use crate::{context::Context, utility};

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
	pub fn new(ctx: &Context) -> AppInfo {
		AppInfo {
			api_version: ctx.loader.instance_version(),
			engine_name: String::new(),
			engine_name_c: std::ffi::CString::default(),
			engine_version: 0,
			app_name: String::new(),
			app_name_c: std::ffi::CString::default(),
			app_version: 0,
		}
	}

	pub fn api_version(&self) -> String {
		utility::as_version_string(&self.api_version)
	}

	pub fn engine(mut self, name: &str, version: u32) -> AppInfo {
		self.engine_name = String::from(name);
		self.engine_name_c = std::ffi::CString::new(name).unwrap();
		self.engine_version = version;
		self
	}

	pub fn engine_version(&self) -> String {
		utility::as_version_string(&self.engine_version)
	}

	pub fn application(mut self, name: &str, version: u32) -> AppInfo {
		self.app_name = String::from(name);
		self.app_name_c = std::ffi::CString::new(name).unwrap();
		self.app_version = version;
		self
	}

	pub fn app_version(&self) -> String {
		utility::as_version_string(&self.app_version)
	}

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

	pub fn to_vk(&self) -> erupt::vk::ApplicationInfo {
		erupt::vk::ApplicationInfoBuilder::new()
			.api_version(self.api_version)
			.engine_name(&self.engine_name_c)
			.engine_version(self.engine_version)
			.application_name(&self.app_name_c)
			.application_version(self.app_version)
			.build()
	}
}
