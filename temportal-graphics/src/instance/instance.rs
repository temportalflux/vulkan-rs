use crate::{
	backend,
	device::physical,
	general::Surface,
	utility::{self, VulkanObject},
	Context,
};

use raw_window_handle;
use std::rc::Rc;

/// A user-owned singleton for the [`Vulkan Instance`](backend::InstanceLoader)
pub struct Instance {
	debug_messenger: Option<backend::vk::DebugUtilsMessengerEXT>,
	surface_ext: backend::extensions::khr::Surface,
	debug_ext: backend::extensions::ext::DebugUtils,
	internal: backend::Instance,
}

impl Instance {
	/// The internal constructor. Users should use [`Info.create_object`](struct.Info.html#method.create_object) to create a vulkan instance.
	pub fn from(
		internal: backend::Instance,
		ctx: &Context,
		enable_validation: bool,
	) -> utility::Result<Instance> {
		let mut instance = Instance {
			surface_ext: backend::extensions::khr::Surface::new(&ctx.loader, &internal),
			debug_ext: backend::extensions::ext::DebugUtils::new(&ctx.loader, &internal),
			internal,
			debug_messenger: None,
		};

		if enable_validation {
			let messenger_info = backend::vk::DebugUtilsMessengerCreateInfoEXT::builder()
				.message_severity(
					backend::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
						| backend::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING, //| backend::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT
				)
				.message_type(
					backend::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
						| backend::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
						| backend::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL,
				)
				.pfn_user_callback(Some(debug_callback));
			instance.debug_messenger = Some(utility::as_vulkan_error(unsafe {
				instance
					.debug_ext
					.create_debug_utils_messenger(&messenger_info, None)
			})?);
		}

		Ok(instance)
	}

	/// Creates a vulkan [`Surface`] using a window handle the user provides.
	pub fn create_surface(
		context: &Context,
		instance: &Rc<Self>,
		handle: &impl raw_window_handle::HasRawWindowHandle,
	) -> utility::Result<Surface> {
		utility::as_vulkan_error(unsafe {
			ash_window::create_surface(&context.loader, &instance.internal, handle, None)
		})
		.map(|ok| Surface::from(instance.clone(), ok))
	}

	#[doc(hidden)]
	pub fn destroy_surface(&self, value: backend::vk::SurfaceKHR) {
		unsafe {
			self.surface_ext.destroy_surface(value, None);
		}
	}

	/// Searches for an applicable [`Device`](crate::device::physical::Device) that fits the provided constraints and surface.
	pub fn find_physical_device(
		instance: &Rc<Instance>,
		constraints: &Vec<physical::Constraint>,
		surface: &Rc<Surface>,
	) -> Result<physical::Device, Option<physical::Constraint>> {
		use ash::version::InstanceV1_0;
		match unsafe { instance.internal.enumerate_physical_devices() }
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

/// A trait exposing the internal value for the wrapped [`backend::Instance`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::Instance> for Instance {
	fn unwrap(&self) -> &backend::Instance {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::Instance {
		&mut self.internal
	}
}

impl Drop for Instance {
	fn drop(&mut self) {
		use ash::version::InstanceV1_0;
		unsafe {
			if let Some(msgr) = self.debug_messenger {
				self.debug_ext.destroy_debug_utils_messenger(msgr, None);
			}
			self.internal.destroy_instance(None);
		}
	}
}

#[doc(hidden)]
impl Instance {
	pub fn get_physical_device_properties(
		&self,
		device: &backend::vk::PhysicalDevice,
	) -> backend::vk::PhysicalDeviceProperties {
		use ash::version::InstanceV1_0;
		unsafe { self.internal.get_physical_device_properties(*device) }
	}

	pub fn get_physical_device_queue_family_properties(
		&self,
		device: &backend::vk::PhysicalDevice,
	) -> Vec<backend::vk::QueueFamilyProperties> {
		use ash::version::InstanceV1_0;
		unsafe {
			self.internal
				.get_physical_device_queue_family_properties(*device)
		}
	}

	pub fn does_physical_device_surface_support_khr(
		&self,
		device: &backend::vk::PhysicalDevice,
		queue_family_index: usize,
		surface: &backend::vk::SurfaceKHR,
	) -> bool {
		unsafe {
			self.surface_ext.get_physical_device_surface_support(
				*device,
				queue_family_index as u32,
				*surface,
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
			self.surface_ext
				.get_physical_device_surface_formats(*device, *surface)
		}
		.unwrap()
	}

	pub fn get_physical_device_surface_present_modes(
		&self,
		device: &backend::vk::PhysicalDevice,
		surface: &backend::vk::SurfaceKHR,
	) -> Vec<backend::vk::PresentModeKHR> {
		unsafe {
			self.surface_ext
				.get_physical_device_surface_present_modes(*device, *surface)
		}
		.unwrap()
	}

	pub fn enumerate_device_extension_properties(
		&self,
		device: &backend::vk::PhysicalDevice,
	) -> Vec<backend::vk::ExtensionProperties> {
		unsafe {
			use ash::version::InstanceV1_0;
			self.internal.enumerate_device_extension_properties(*device)
		}
		.unwrap()
	}

	pub fn get_physical_device_surface_capabilities(
		&self,
		device: &backend::vk::PhysicalDevice,
		surface: &backend::vk::SurfaceKHR,
	) -> backend::vk::SurfaceCapabilitiesKHR {
		unsafe {
			self.surface_ext
				.get_physical_device_surface_capabilities(*device, *surface)
		}
		.unwrap()
	}
}

#[doc(hidden)]
unsafe extern "system" fn debug_callback(
	message_severity: backend::vk::DebugUtilsMessageSeverityFlagsEXT,
	_message_types: backend::vk::DebugUtilsMessageTypeFlagsEXT,
	p_callback_data: *const backend::vk::DebugUtilsMessengerCallbackDataEXT,
	_p_user_data: *mut std::ffi::c_void,
) -> backend::vk::Bool32 {
	let log_level = match message_severity {
		backend::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => log::Level::Trace,
		backend::vk::DebugUtilsMessageSeverityFlagsEXT::INFO => log::Level::Info,
		backend::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => log::Level::Warn,
		backend::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => log::Level::Error,
		_ => log::Level::Debug,
	};
	let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message).to_string_lossy();
	log::log!(target: "vulkan", log_level, "{}", message);
	backend::vk::FALSE
}
