use crate::{
	backend,
	device::physical,
	general::Surface,
	utility::{self, VulkanObject},
};

use raw_window_handle;
use std::rc::Rc;

/// A user-owned singleton for the [`Vulkan Instance`](backend::InstanceLoader)
pub struct Instance {
	_internal: backend::InstanceLoader,
	debug_messenger: Option<backend::extensions::ext_debug_utils::DebugUtilsMessengerEXT>,
}

impl Instance {
	/// The internal constructor. Users should use [`Info.create_object`](struct.Info.html#method.create_object) to create a vulkan instance.
	pub fn from(
		_internal: backend::InstanceLoader,
		enable_validation: bool,
	) -> utility::Result<Instance> {
		let mut instance = Instance {
			_internal,
			debug_messenger: None,
		};

		if enable_validation {
			let messenger_info = backend::vk::DebugUtilsMessengerCreateInfoEXTBuilder::new()
				.message_severity(
					backend::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT
						| backend::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT, //| backend::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT
				)
				.message_type(
					backend::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT
						| backend::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT
						| backend::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL_EXT,
				)
				.pfn_user_callback(Some(debug_callback));
			instance.debug_messenger = Some(utility::as_vulkan_error(unsafe {
				instance
					._internal
					.create_debug_utils_messenger_ext(&messenger_info, None, None)
			})?);
		}

		Ok(instance)
	}

	/// Creates a vulkan [`Surface`] using a window handle the user provides.
	pub fn create_surface(
		instance: &Rc<Self>,
		handle: &impl raw_window_handle::HasRawWindowHandle,
	) -> utility::Result<Surface> {
		utility::as_vulkan_error(unsafe {
			backend::utils::surface::create_surface(&instance._internal, handle, None)
		})
		.map(|ok| Surface::from(instance.clone(), ok))
	}

	#[doc(hidden)]
	pub fn destroy_surface(&self, value: backend::vk::SurfaceKHR) {
		unsafe {
			self._internal.destroy_surface_khr(Some(value), None);
		}
	}

	/// Searches for an applicable [`Device`](crate::device::physical::Device) that fits the provided constraints and surface.
	pub fn find_physical_device(
		instance: &Rc<Instance>,
		constraints: &Vec<physical::Constraint>,
		surface: &Rc<Surface>,
	) -> Result<physical::Device, Option<physical::Constraint>> {
		match unsafe { instance._internal.enumerate_physical_devices(None) }
			.unwrap()
			.into_iter()
			.map(|vk_physical_device| {
				physical::Device::from(instance, vk_physical_device, &surface)
			})
			.map(|mut physical_device| {
				match physical_device.score_against_constraints(&constraints, false) {
					Ok(score) => (physical_device, score, None),
					Err(failed_constraint) => (physical_device, 0, Some(failed_constraint)),
				}
			})
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

/// A trait exposing the internal value for the wrapped [`backend::InstanceLoader`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::InstanceLoader> for Instance {
	fn unwrap(&self) -> &backend::InstanceLoader {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::InstanceLoader {
		&mut self._internal
	}
}

impl Drop for Instance {
	fn drop(&mut self) {
		unsafe {
			if let Some(msgr) = self.debug_messenger {
				self._internal
					.destroy_debug_utils_messenger_ext(Some(msgr), None);
			}
			self._internal.destroy_instance(None);
		}
	}
}

#[doc(hidden)]
impl Instance {
	pub fn get_physical_device_properties(
		&self,
		device: &backend::vk::PhysicalDevice,
	) -> backend::vk::PhysicalDeviceProperties {
		unsafe { self._internal.get_physical_device_properties(*device, None) }
	}

	pub fn get_physical_device_queue_family_properties(
		&self,
		device: &backend::vk::PhysicalDevice,
	) -> Vec<backend::vk::QueueFamilyProperties> {
		unsafe {
			self._internal
				.get_physical_device_queue_family_properties(*device, None)
		}
	}

	pub fn does_physical_device_surface_support_khr(
		&self,
		device: &backend::vk::PhysicalDevice,
		queue_family_index: usize,
		surface: &backend::vk::SurfaceKHR,
	) -> bool {
		unsafe {
			self._internal.get_physical_device_surface_support_khr(
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
		device: &backend::vk::PhysicalDevice,
		surface: &backend::vk::SurfaceKHR,
	) -> Vec<backend::vk::SurfaceFormatKHR> {
		unsafe {
			self._internal
				.get_physical_device_surface_formats_khr(*device, *surface, None)
		}
		.unwrap()
	}

	pub fn get_physical_device_surface_present_modes(
		&self,
		device: &backend::vk::PhysicalDevice,
		surface: &backend::vk::SurfaceKHR,
	) -> Vec<backend::vk::PresentModeKHR> {
		unsafe {
			self._internal
				.get_physical_device_surface_present_modes_khr(*device, *surface, None)
		}
		.unwrap()
	}

	pub fn enumerate_device_extension_properties(
		&self,
		device: &backend::vk::PhysicalDevice,
	) -> Vec<backend::vk::ExtensionProperties> {
		unsafe {
			self._internal
				.enumerate_device_extension_properties(*device, None, None)
		}
		.unwrap()
	}

	pub fn get_physical_device_surface_capabilities(
		&self,
		device: &backend::vk::PhysicalDevice,
		surface: &backend::vk::SurfaceKHR,
	) -> backend::vk::SurfaceCapabilitiesKHR {
		unsafe {
			self._internal
				.get_physical_device_surface_capabilities_khr(*device, *surface, None)
		}
		.unwrap()
	}
}

#[doc(hidden)]
unsafe extern "system" fn debug_callback(
	message_severity: backend::vk::DebugUtilsMessageSeverityFlagBitsEXT,
	_message_types: backend::vk::DebugUtilsMessageTypeFlagsEXT,
	p_callback_data: *const backend::vk::DebugUtilsMessengerCallbackDataEXT,
	_p_user_data: *mut std::ffi::c_void,
) -> backend::vk::Bool32 {
	let log_level = match message_severity {
		backend::vk::DebugUtilsMessageSeverityFlagBitsEXT::VERBOSE_EXT => log::Level::Trace,
		backend::vk::DebugUtilsMessageSeverityFlagBitsEXT::INFO_EXT => log::Level::Info,
		backend::vk::DebugUtilsMessageSeverityFlagBitsEXT::WARNING_EXT => log::Level::Warn,
		backend::vk::DebugUtilsMessageSeverityFlagBitsEXT::ERROR_EXT => log::Level::Error,
		_ => log::Level::Debug,
	};
	let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message).to_string_lossy();
	log::log!(target: "vulkan", log_level, "{}", message);
	backend::vk::FALSE
}
