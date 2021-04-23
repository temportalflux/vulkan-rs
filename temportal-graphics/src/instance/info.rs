use crate::{
	context::Context,
	instance,
	utility::{self, VulkanInfo, VulkanInfoMut},
	AppInfo,
};
use erupt;

/// Information used to construct a [`Vulkan Instance`](instance::Instance).
#[derive(Debug)]
pub struct Info {
	app_info: AppInfo,
	extensions: Vec<String>,
	layers: Vec<String>,
	validation_enabled: bool,

	app_info_raw: erupt::vk::ApplicationInfo,
	extensions_raw: Vec<utility::CStrPtr>,
	layers_raw: Vec<utility::CStrPtr>,
}

impl Default for Info {
	fn default() -> Info {
		Info {
			app_info: AppInfo::default(),
			extensions: Vec::new(),
			layers: Vec::new(),
			validation_enabled: false,

			app_info_raw: erupt::vk::ApplicationInfo::default(),
			extensions_raw: Vec::new(),
			layers_raw: Vec::new(),
		}
	}
}

impl Info {
	/// Set information about the application creating the instance.
	pub fn set_app_info(mut self, info: AppInfo) -> Self {
		self.app_info = info;
		self
	}

	#[doc(hidden)]
	fn append_raw_extensions(&mut self, exts: Vec<utility::CStrPtr>) {
		for ext in exts.into_iter() {
			self.add_raw_extension(ext);
		}
	}

	#[doc(hidden)]
	fn add_raw_extension(&mut self, raw: utility::CStrPtr) {
		self.extensions.push(
			unsafe { std::ffi::CStr::from_ptr(raw) }
				.to_owned()
				.into_string()
				.unwrap(),
		);
	}

	/// Adds the name of an extension to the list of required extensions for the instance.
	pub fn add_extension(&mut self, name: &str) {
		self.extensions.push(
			std::ffi::CString::new(name.as_bytes())
				.unwrap()
				.into_string()
				.unwrap(),
		);
	}

	/// Adds the name of an layer to the list of required layers for the instance.
	pub fn add_layer(&mut self, name: &str) {
		self.layers.push(
			std::ffi::CString::new(name.as_bytes())
				.unwrap()
				.into_string()
				.unwrap(),
		);
	}

	/// Formats a string with the application info, extension names, and layer names.
	pub fn description(&self) -> String {
		format!(
			"{} with extensions {:?} and layers {:?}",
			self.app_info.description(),
			self.extensions,
			self.layers
		)
	}

	/// Returns true if any of the layers in the info are not valid for a given context.
	pub fn has_invalid_layer(&self, ctx: &Context) -> Option<String> {
		for layer in self.layers.iter() {
			if !ctx.is_valid_instance_layer(&layer) {
				return Some(layer.clone());
			}
		}
		None
	}

	/// Sets the window
	pub fn set_window(
		mut self,
		window_handle: &impl raw_window_handle::HasRawWindowHandle,
	) -> Self {
		use erupt::utils::surface::enumerate_required_extensions;
		let window_extensions = enumerate_required_extensions(window_handle).unwrap();
		self.append_raw_extensions(window_extensions);
		self
	}

	/// Sets if the instance uses validation.
	pub fn set_use_validation(mut self, enable_validation: bool) -> Self {
		self.validation_enabled = enable_validation;
		if enable_validation {
			self.add_extension("VK_EXT_debug_utils");
			self.add_layer("VK_LAYER_KHRONOS_validation");
		}
		self
	}

	/// Creates the vulkan instance object, thereby consuming the info.
	pub fn create_object(mut self, ctx: &Context) -> utility::Result<instance::Instance> {
		log::info!(target: crate::LOG, "Initializing {}", self.description());
		log::debug!(
			target: crate::LOG,
			"Available extensions: {:?}",
			ctx.valid_instance_extensions
		);
		log::debug!(
			target: crate::LOG,
			"Available layers: {:?}",
			ctx.valid_instance_layers
		);
		if let Some(layer) = self.has_invalid_layer(&ctx) {
			return Err(utility::Error::InvalidInstanceLayer(layer));
		}
		let create_info = self.to_vk();
		let instance_loader = match erupt::InstanceLoader::new(&ctx.loader, &create_info, None) {
			Ok(inst) => inst,
			Err(err) => match err {
				erupt::LoaderError::VulkanError(res) => {
					return Err(utility::Error::VulkanError(res))
				}
				erupt::LoaderError::SymbolNotAvailable => {
					return Err(utility::Error::InstanceSymbolNotAvailable())
				}
			},
		};
		instance::Instance::from(instance_loader, self.validation_enabled)
	}
}

impl utility::VulkanInfoMut<erupt::vk::InstanceCreateInfo> for Info {
	/// Converts the [`Info`] into the [`erupt::vk::InstanceCreateInfo`] struct
	/// used to create a [`instance::Instance`].
	fn to_vk(&mut self) -> erupt::vk::InstanceCreateInfo {
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
