use crate::{backend, device::physical, flags, general::Surface, utility, Context};

use raw_window_handle;
use std::sync;

/// A user-owned singleton for the [`Vulkan Instance`](backend::Instance)
pub struct Instance {
	debug_messenger: Option<backend::vk::DebugUtilsMessengerEXT>,
	surface_ext: backend::extensions::khr::Surface,
	debug_ext: backend::extensions::ext::DebugUtils,
	internal: backend::Instance,
}

impl Instance {
	/// The internal constructor. Users should use [`Info.create_object`](struct.Info.html#method.create_object) to create a vulkan instance.
	pub(crate) fn from(
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
						| backend::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
						| backend::vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
				)
				.message_type(
					backend::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
						| backend::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
						| backend::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL,
				)
				.pfn_user_callback(Some(debug_callback));
			instance.debug_messenger = Some(unsafe {
				instance
					.debug_ext
					.create_debug_utils_messenger(&messenger_info, None)
			}?);
		}

		Ok(instance)
	}

	/// Creates a vulkan [`Surface`] using a window handle the user provides.
	pub fn create_surface(
		context: &Context,
		instance: &sync::Arc<Self>,
		display_handle: raw_window_handle::RawDisplayHandle,
		window_handle: raw_window_handle::RawWindowHandle,
	) -> utility::Result<Surface> {
		Ok(unsafe {
			ash_window::create_surface(
				&context.loader,
				&instance.internal,
				display_handle,
				window_handle,
				None,
			)
		}
		.map(|ok| Surface::from(instance.clone(), ok))?)
	}

	#[doc(hidden)]
	pub fn destroy_surface(&self, value: backend::vk::SurfaceKHR) {
		unsafe {
			self.surface_ext.destroy_surface(value, None);
		}
	}

	/// Searches for an applicable [`Device`](crate::device::physical::Device) that fits the provided constraints and surface.
	pub fn find_physical_device(
		instance: &sync::Arc<Instance>,
		constraints: &Vec<physical::Constraint>,
		surface: &sync::Arc<Surface>,
	) -> Result<physical::Device, Option<physical::Constraint>> {
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

	pub fn debug_utils(&self) -> &backend::extensions::ext::DebugUtils {
		&self.debug_ext
	}
}

impl std::ops::Deref for Instance {
	type Target = backend::Instance;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Instance {
	fn drop(&mut self) {
		unsafe {
			if let Some(msgr) = self.debug_messenger {
				self.debug_ext.destroy_debug_utils_messenger(msgr, None);
			}
			self.internal.destroy_instance(None);
		}
	}
}

impl utility::HandledObject for Instance {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Instance as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.handle().as_raw()
	}
}

#[doc(hidden)]
impl Instance {
	pub fn get_physical_device_properties(
		&self,
		device: &backend::vk::PhysicalDevice,
	) -> backend::vk::PhysicalDeviceProperties {
		unsafe { self.internal.get_physical_device_properties(*device) }
	}

	pub fn get_physical_device_queue_family_properties(
		&self,
		device: &backend::vk::PhysicalDevice,
	) -> Vec<backend::vk::QueueFamilyProperties> {
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
		unsafe { self.internal.enumerate_device_extension_properties(*device) }.unwrap()
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

	pub fn get_physical_device_format_properties(
		&self,
		device: &backend::vk::PhysicalDevice,
		format: flags::format::Format,
	) -> backend::vk::FormatProperties {
		unsafe {
			self.internal
				.get_physical_device_format_properties(*device, format)
		}
	}
}

#[doc(hidden)]
unsafe extern "system" fn debug_callback(
	severity: backend::vk::DebugUtilsMessageSeverityFlagsEXT,
	kind: backend::vk::DebugUtilsMessageTypeFlagsEXT,
	p_callback_data: *const backend::vk::DebugUtilsMessengerCallbackDataEXT,
	_p_user_data: *mut std::ffi::c_void,
) -> backend::vk::Bool32 {
	let log_level = match severity {
		backend::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => log::Level::Trace,
		backend::vk::DebugUtilsMessageSeverityFlagsEXT::INFO => log::Level::Info,
		backend::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => log::Level::Warn,
		backend::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => log::Level::Error,
		_ => log::Level::Debug,
	};
	let target = format!(
		"vulkan-{}",
		match kind {
			backend::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "general",
			backend::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "performance",
			backend::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "validation",
			_ => "unknown",
		}
	);
	let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message).to_string_lossy();
	log::log!(target: &target, log_level, "{}", message);
	backend::vk::FALSE
}
