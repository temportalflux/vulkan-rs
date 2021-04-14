use erupt;
use crate::{utility, AppInfo, context::Context};

#[derive(Debug)]
pub struct Info {
	app_info: AppInfo,
	extensions: Vec<String>,
	layers: Vec<String>,
	app_info_raw: erupt::vk::ApplicationInfo,
	extensions_raw: Vec<utility::CStrPtr>,
	layers_raw: Vec<utility::CStrPtr>,
}

impl Info {
	pub fn new() -> Info {
		Info {
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

	pub fn append_raw_extensions(&mut self, exts: Vec<utility::CStrPtr>) {
		for ext in exts.into_iter() {
			self.add_raw_extension(ext);
		}
	}

	pub fn add_raw_extension(&mut self, raw: utility::CStrPtr) {
		self.extensions.push(
			unsafe { std::ffi::CStr::from_ptr(raw) }
				.to_owned()
				.into_string()
				.unwrap(),
		);
	}

	pub fn add_raw_layer(&mut self, raw: utility::CStrPtr) {
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

	pub fn description(&self) -> String {
		format!(
			"{} with extensions {:?} and layers {:?}",
			self.app_info.description(),
			self.extensions,
			self.layers
		)
	}

	pub fn has_invalid_layer(&self, ctx: &Context) -> Option<String> {
		for layer in self.layers.iter() {
			if !ctx.is_valid_instance_layer(&layer) {
				return Some(layer.clone());
			}
		}
		None
	}

	pub fn to_vk(&mut self) -> erupt::vk::InstanceCreateInfo {
		self.app_info_raw = self.app_info.to_vk();
		self.extensions_raw = self
			.extensions
			.iter()
			.map(|owned| utility::to_cstr_ptr(&owned))
			.collect();
		self.layers_raw = self
			.layers
			.iter()
			.map(|owned| utility::to_cstr_ptr(&owned))
			.collect();
		erupt::vk::InstanceCreateInfoBuilder::new()
			.application_info(&self.app_info_raw)
			.enabled_extension_names(&self.extensions_raw)
			.enabled_layer_names(&self.layers_raw)
			.build()
	}
}

#[derive(Debug)]
pub enum Error {
	InvalidInstanceLayer(String),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Error::InvalidInstanceLayer(ref layer_name) => {
				write!(f, "Invalid vulkan instance layer: {}", layer_name)
			}
		}
	}
}

impl std::error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::InvalidInstanceLayer(ref layer_name) => layer_name.as_str(),
		}
	}
}
