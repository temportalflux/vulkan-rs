use erupt;
use raw_window_handle;

use crate::{context::Context, physical, instance};

pub struct Instance {
	internal: erupt::InstanceLoader,
	debug_messenger: Option<erupt::extensions::ext_debug_utils::DebugUtilsMessengerEXT>,
}

impl Instance {
	pub fn new(
		ctx: &Context,
		info: &mut instance::Info,
		is_validation_enabled: bool,
	) -> Result<Instance, Box<dyn std::error::Error>> {
		println!("Initializing {}", info.description());
		println!("Available extensions: {:?}", ctx.valid_instance_extensions);
		println!("Available layers: {:?}", ctx.valid_instance_layers);
		if let Some(layer) = info.has_invalid_layer(&ctx) {
			return Result::Err(Box::new(instance::Error::InvalidInstanceLayer(layer)));
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
		constraints: &Vec<physical::Constraint>,
		surface: &erupt::vk::SurfaceKHR,
	) -> Result<physical::Device, Option<physical::Constraint>> {
		match unsafe { self.internal.enumerate_physical_devices(None) }
			.unwrap()
			.into_iter()
			.map(|vk_physical_device| physical::Device::new(self, vk_physical_device, &surface))
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
