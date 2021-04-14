use super::context::Context;
use erupt;
use raw_window_handle;

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

type CStrPtr = *const ::std::os::raw::c_char;
pub fn to_cstr_ptr(name: &String) -> CStrPtr {
	name.as_str() as *const str as CStrPtr
}

impl AppInfo {
	pub fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
		erupt::vk::make_version(major, minor, patch)
	}

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

	fn as_version_string(&self, version: &u32) -> String {
		format!(
			"{}.{}.{}",
			erupt::vk::version_major(*version),
			erupt::vk::version_minor(*version),
			erupt::vk::version_patch(*version)
		)
	}

	pub fn api_version(&self) -> String {
		self.as_version_string(&self.api_version)
	}

	pub fn engine(mut self, name: &str, version: u32) -> AppInfo {
		self.engine_name = String::from(name);
		self.engine_name_c = std::ffi::CString::new(name).unwrap();
		self.engine_version = version;
		self
	}

	pub fn engine_version(&self) -> String {
		self.as_version_string(&self.engine_version)
	}

	pub fn application(mut self, name: &str, version: u32) -> AppInfo {
		self.app_name = String::from(name);
		self.app_name_c = std::ffi::CString::new(name).unwrap();
		self.app_version = version;
		self
	}

	pub fn app_version(&self) -> String {
		self.as_version_string(&self.app_version)
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

#[derive(Debug)]
pub struct InstanceInfo {
	app_info: AppInfo,
	extensions: Vec<String>,
	layers: Vec<String>,
	
	app_info_raw: erupt::vk::ApplicationInfo,
	extensions_raw: Vec<CStrPtr>,
	layers_raw: Vec<CStrPtr>,
}

impl InstanceInfo {
	pub fn new() -> InstanceInfo {
		InstanceInfo {
			app_info: AppInfo::default(),
			app_info_raw: erupt::vk::ApplicationInfo::default(),
			extensions: Vec::new(),
			extensions_raw: Vec::new(),
			layers: Vec::new(),
			layers_raw: Vec::new(),
		}
	}

	pub fn app_info(mut self, info: AppInfo) -> Self {
		self.app_info = info;
		self
	}

	pub fn append_raw_extensions(&mut self, exts: Vec<CStrPtr>) {
		for ext in exts.into_iter() {
			self.add_raw_extension(ext);
		}
	}

	pub fn add_raw_extension(&mut self, raw: CStrPtr) {
		self.extensions.push(
			unsafe { std::ffi::CStr::from_ptr(raw) }
				.to_owned()
				.into_string()
				.unwrap(),
		);
	}

	pub fn add_raw_layer(&mut self, raw: CStrPtr) {
		self.layers.push(
			unsafe { std::ffi::CStr::from_ptr(raw) }
				.to_owned()
				.into_string()
				.unwrap(),
		);
	}

	pub fn add_extension(&mut self, name: &str) {
		self.extensions.push(std::ffi::CString::new(name.as_bytes()).unwrap().into_string().unwrap());
	}

	pub fn add_layer(&mut self, name: &str) {
		self.layers.push(std::ffi::CString::new(name.as_bytes()).unwrap().into_string().unwrap());
	}

	fn to_vk(&mut self) -> erupt::vk::InstanceCreateInfo {
		self.app_info_raw = self.app_info.to_vk();
		self.extensions_raw = self
			.extensions
			.iter()
			.map(|owned| to_cstr_ptr(&owned))
			.collect();
		self.layers_raw = self
			.layers
			.iter()
			.map(|owned| to_cstr_ptr(&owned))
			.collect();
		erupt::vk::InstanceCreateInfoBuilder::new()
			.application_info(&self.app_info_raw)
			.enabled_extension_names(&self.extensions_raw)
			.enabled_layer_names(&self.layers_raw)
			.build()
	}
}

#[derive(Debug)]
pub enum InstanceError {
	InvalidInstanceLayer(String),
}

impl std::fmt::Display for InstanceError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			InstanceError::InvalidInstanceLayer(ref layer_name) => {
				write!(f, "Invalid vulkan instance layer: {}", layer_name)
			}
		}
	}
}

impl std::error::Error for InstanceError {
	fn description(&self) -> &str {
		match *self {
			InstanceError::InvalidInstanceLayer(ref layer_name) => layer_name.as_str(),
		}
	}
}

pub struct Instance {
	internal: erupt::InstanceLoader,
}

impl Instance {
	pub fn new(
		ctx: &Context,
		info: &mut InstanceInfo,
	) -> Result<Instance, Box<dyn std::error::Error>> {
		println!(
			"Initializing {} with extensions {:?} and layers {:?}",
			info.app_info.description(),
			info.extensions,
			info.layers
		);
		println!("Available extensions: {:?}", ctx.valid_instance_extensions);
		println!("Available layers: {:?}", ctx.valid_instance_layers);
		for layer in info.layers.iter() {
			if !ctx.is_valid_instance_layer(&layer) {
				return Result::Err(Box::new(InstanceError::InvalidInstanceLayer(layer.clone())));
			}
		}
		let instance_create_info: erupt::vk::InstanceCreateInfo = info.to_vk();
		let instance_loader = erupt::InstanceLoader::new(&ctx.loader, &instance_create_info, None)?;
		Ok(Instance {
			internal: instance_loader,
		})
	}

	pub fn create_surface(
		&self,
		handle: &impl raw_window_handle::HasRawWindowHandle,
	) -> erupt::vk::SurfaceKHR {
		unsafe { erupt::utils::surface::create_surface(&self.internal, handle, None) }.unwrap()
	}
}
