use super::context::Context;
use super::*;
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

pub fn as_version_string(version: &u32) -> String {
	format!(
		"{}.{}.{}",
		erupt::vk::version_major(*version),
		erupt::vk::version_minor(*version),
		erupt::vk::version_patch(*version)
	)
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

	pub fn api_version(&self) -> String {
		as_version_string(&self.api_version)
	}

	pub fn engine(mut self, name: &str, version: u32) -> AppInfo {
		self.engine_name = String::from(name);
		self.engine_name_c = std::ffi::CString::new(name).unwrap();
		self.engine_version = version;
		self
	}

	pub fn engine_version(&self) -> String {
		as_version_string(&self.engine_version)
	}

	pub fn application(mut self, name: &str, version: u32) -> AppInfo {
		self.app_name = String::from(name);
		self.app_name_c = std::ffi::CString::new(name).unwrap();
		self.app_version = version;
		self
	}

	pub fn app_version(&self) -> String {
		as_version_string(&self.app_version)
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
		self.extensions.push(
			std::ffi::CString::new(name.as_bytes())
				.unwrap()
				.into_string()
				.unwrap(),
		);
	}

	pub fn add_layer(&mut self, name: &str) {
		self.layers.push(
			std::ffi::CString::new(name.as_bytes())
				.unwrap()
				.into_string()
				.unwrap(),
		);
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
	debug_messenger: Option<erupt::extensions::ext_debug_utils::DebugUtilsMessengerEXT>,
}

impl Instance {
	pub fn new(
		ctx: &Context,
		info: &mut InstanceInfo,
		is_validation_enabled: bool,
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

		let mut instance = Instance {
			internal: instance_loader,
			debug_messenger: None,
		};

		if is_validation_enabled {
			let messenger_info = erupt::vk::DebugUtilsMessengerCreateInfoEXTBuilder::new()
				.message_severity(
					erupt::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT
						| erupt::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT
						| erupt::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT,
				)
				.message_type(
					erupt::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL_EXT
						| erupt::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT
						| erupt::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT,
				)
				.pfn_user_callback(Some(debug_callback));
			instance.debug_messenger = Some(
				unsafe {
					instance
						.internal
						.create_debug_utils_messenger_ext(&messenger_info, None, None)
				}
				.unwrap(),
			);
		}

		Ok(instance)
	}

	pub fn create_surface(
		&self,
		handle: &impl raw_window_handle::HasRawWindowHandle,
	) -> erupt::vk::SurfaceKHR {
		unsafe { erupt::utils::surface::create_surface(&self.internal, handle, None) }.unwrap()
	}

	pub fn find_physical_device(
		&self,
		constraints: &Vec<PhysicalDeviceConstraint>,
		surface: &erupt::vk::SurfaceKHR,
	) -> Result<PhysicalDevice, Option<PhysicalDeviceConstraint>> {
		match unsafe { self.internal.enumerate_physical_devices(None) }
			.unwrap()
			.into_iter()
			.map(|vk_physical_device| PhysicalDevice::new(self, vk_physical_device, &surface))
			.map(
				|physical_device| match physical_device.score_against_constraints(&constraints) {
					Ok(score) => (physical_device, score, None),
					Err(failed_constraint) => (physical_device, 0, Some(failed_constraint)),
				},
			)
			.max_by_key(|(_, score, _)| *score)
		{
			Some((device, _, failed_constraint)) => match failed_constraint {
				None => Ok(device),
				Some(constraint_that_failed) => Err(Some(constraint_that_failed)),
			},
			None => Err(None),
		}
	}
}

impl Instance {
	pub fn get_physical_device_properties(
		&self,
		device: &erupt::vk::PhysicalDevice,
	) -> erupt::vk::PhysicalDeviceProperties {
		unsafe { self.internal.get_physical_device_properties(*device, None) }
	}

	pub fn get_physical_device_queue_family_properties(
		&self,
		device: &erupt::vk::PhysicalDevice,
	) -> Vec<erupt::vk::QueueFamilyProperties> {
		unsafe {
			self.internal
				.get_physical_device_queue_family_properties(*device, None)
		}
	}

	pub fn does_physical_device_surface_support_khr(
		&self,
		device: &erupt::vk::PhysicalDevice,
		queue_family_index: usize,
		surface: &erupt::vk::SurfaceKHR,
	) -> bool {
		unsafe {
			self.internal.get_physical_device_surface_support_khr(
				*device,
				queue_family_index as u32,
				*surface,
				None,
			)
		}
		.unwrap()
	}

	pub fn get_physical_device_surface_formats(
		&self,
		device: &erupt::vk::PhysicalDevice,
		surface: &erupt::vk::SurfaceKHR,
	) -> Vec<erupt::vk::SurfaceFormatKHR> {
		unsafe {
			self.internal
				.get_physical_device_surface_formats_khr(*device, *surface, None)
		}
		.unwrap()
	}

	pub fn get_physical_device_surface_present_modes(
		&self,
		device: &erupt::vk::PhysicalDevice,
		surface: &erupt::vk::SurfaceKHR,
	) -> Vec<erupt::vk::PresentModeKHR> {
		unsafe {
			self.internal
				.get_physical_device_surface_present_modes_khr(*device, *surface, None)
		}
		.unwrap()
	}

	pub fn enumerate_device_extension_properties(
		&self,
		device: &erupt::vk::PhysicalDevice,
	) -> Vec<erupt::vk::ExtensionProperties> {
		unsafe {
			self.internal
				.enumerate_device_extension_properties(*device, None, None)
		}
		.unwrap()
	}
}

unsafe extern "system" fn debug_callback(
	_message_severity: erupt::vk::DebugUtilsMessageSeverityFlagBitsEXT,
	_message_types: erupt::vk::DebugUtilsMessageTypeFlagsEXT,
	p_callback_data: *const erupt::vk::DebugUtilsMessengerCallbackDataEXT,
	_p_user_data: *mut std::ffi::c_void,
) -> erupt::vk::Bool32 {
	eprintln!(
		"{}",
		std::ffi::CStr::from_ptr((*p_callback_data).p_message).to_string_lossy()
	);

	erupt::vk::FALSE
}
