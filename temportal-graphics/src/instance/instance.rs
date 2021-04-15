use erupt;
use raw_window_handle;

use crate::{device::physical, general::Surface, utility::VulkanObject};

/// A user-owned singleton for the [`Vulkan Instance`](erupt::InstanceLoader)
pub struct Instance {
	internal: erupt::InstanceLoader,
	debug_messenger: Option<erupt::extensions::ext_debug_utils::DebugUtilsMessengerEXT>,
}

impl Instance {
	/// The internal constructor. Users should use [`Info.create_object`](struct.Info.html#method.create_object) to create a vulkan instance.
	pub fn from(
		internal: erupt::InstanceLoader,
		enable_validation: bool,
	) -> Result<Instance, Box<dyn std::error::Error>> {
		let mut instance = Instance {
			internal,
			debug_messenger: None,
		};

		if enable_validation {
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

	/// Creates a vulkan [`Surface`] using a window handle the user provides.
	pub fn create_surface(&self, handle: &impl raw_window_handle::HasRawWindowHandle) -> Surface {
		Surface::from(
			unsafe { erupt::utils::surface::create_surface(&self.internal, handle, None) }.unwrap(),
		)
	}

	/// Searches for an applicable [`Device`](../device/physical/struct.Device.html) that fits the provided constraints and surface.
	pub fn find_physical_device(
		&self,
		constraints: &Vec<physical::Constraint>,
		surface: &Surface,
	) -> Result<physical::Device, Option<physical::Constraint>> {
		match unsafe { self.internal.enumerate_physical_devices(None) }
			.unwrap()
			.into_iter()
			.map(|vk_physical_device| {
				physical::Device::from(self, vk_physical_device, &surface.unwrap())
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

/// A trait exposing the internal value for the wrapped [`erupt::InstanceLoader`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::InstanceLoader> for Instance {
	fn unwrap(&self) -> &erupt::InstanceLoader {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::InstanceLoader {
		&mut self.internal
	}
}

#[doc(hidden)]
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

	pub fn get_physical_device_surface_capabilities(
		&self,
		device: &erupt::vk::PhysicalDevice,
		surface: &erupt::vk::SurfaceKHR,
	) -> erupt::vk::SurfaceCapabilitiesKHR {
		unsafe {
			self.internal
				.get_physical_device_surface_capabilities_khr(*device, *surface, None)
		}
		.unwrap()
	}
}

#[doc(hidden)]
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