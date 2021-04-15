use crate::{
	device::physical,
	instance::Instance,
	utility::{self, VulkanObject},
	ColorSpace, Extent2D, Format, PresentMode, QueueFlags, SurfaceTransform,
};
use std::collections::hash_map::HashMap;

pub use erupt::vk::PhysicalDeviceType as Kind;

struct QueueFamily {
	index: usize,
	properties: erupt::vk::QueueFamilyProperties,
	supports_surface: bool,
}

/// The wrapper for [`Vulkan PhysicalDevice`](erupt::vk::PhysicalDevice) objects.
/// Represents a literal GPU.
pub struct Device {
	_internal: erupt::vk::PhysicalDevice,

	properties: erupt::vk::PhysicalDeviceProperties,
	queue_families: Vec<QueueFamily>,
	surface_formats: Vec<erupt::vk::SurfaceFormatKHR>,
	present_modes: Vec<PresentMode>,
	extension_properties: HashMap<String, erupt::vk::ExtensionProperties>,
	surface_capabilities: erupt::vk::SurfaceCapabilitiesKHR,

	pub selected_present_mode: PresentMode,
}

impl Device {
	
	/// The internal constructor. Users should use [`Instance.find_physical_device`](../../instance/struct.Instance.html#method.find_physical_device) to create a vulkan instance.
	pub fn from(
		instance: &Instance,
		vk: erupt::vk::PhysicalDevice,
		surface: &erupt::vk::SurfaceKHR,
	) -> Device {
		Device {
			_internal: vk,
			properties: instance.get_physical_device_properties(&vk),
			queue_families: instance
				.get_physical_device_queue_family_properties(&vk)
				.into_iter()
				.enumerate()
				.map(|(index, properties)| QueueFamily {
					index,
					properties,
					supports_surface: instance
						.does_physical_device_surface_support_khr(&vk, index, &surface),
				})
				.collect(),
			surface_formats: instance.get_physical_device_surface_formats(&vk, &surface),
			present_modes: instance.get_physical_device_surface_present_modes(&vk, &surface),
			extension_properties: instance
				.enumerate_device_extension_properties(&vk)
				.into_iter()
				.map(|prop| {
					(
						unsafe { std::ffi::CStr::from_ptr(&prop.extension_name as *const i8) }
							.to_owned()
							.into_string()
							.unwrap(),
						prop,
					)
				})
				.collect(),
			surface_capabilities: instance.get_physical_device_surface_capabilities(&vk, &surface),
			selected_present_mode: PresentMode::FIFO_KHR,
		}
	}

	/// Returns the kind of graphics processor unit this is (i.e. dedicated, integrated, etc).
	pub fn device_type(&self) -> physical::Kind {
		self.properties.device_type
	}

	/// Returns the descriptive name of the device (i.e. "GeForce RTX 2070").
	pub fn name(&self) -> String {
		unsafe { std::ffi::CStr::from_ptr(&self.properties.device_name as *const i8) }
			.to_owned()
			.into_string()
			.unwrap()
	}

	/// Returns the stringified (i.e. `major.minor.patch`) representation of the devices's api version.
	pub fn api_version(&self) -> String {
		utility::as_version_string(&self.properties.api_version)
	}

	/// Returns the stringified (i.e. `major.minor.patch`) representation of the devices's driver version.
	pub fn driver_version(&self) -> String {
		utility::as_version_string(&self.properties.driver_version)
	}

	/// Returns an optional index representing a queue family which supports specific flags and possibly the surface.
	pub fn get_queue_index(&self, flags: QueueFlags, requires_surface: bool) -> Option<usize> {
		match self.queue_families.iter().find(|family| {
			family.properties.queue_flags.contains(flags)
				&& (!requires_surface || family.supports_surface)
		}) {
			Some(family) => Some(family.index),
			None => None,
		}
	}

	#[doc(hidden)]
	fn contains_all_surface_constraints(&self, format: Format, color_space: ColorSpace) -> bool {
		for supported_format in self.surface_formats.iter() {
			if supported_format.format == format && supported_format.color_space == color_space {
				return true;
			}
		}
		false
	}

	/// Returns a range representing the minimum and maximum number of images that the surface can support for a [`Swapchain`](../swapchain/struct.Swapchain.html).
	pub fn image_count_range(&self) -> std::ops::Range<u32> {
		self.surface_capabilities.min_image_count..self.surface_capabilities.max_image_count
	}

	pub fn image_extent(&self) -> Extent2D {
		self.surface_capabilities.current_extent
	}

	pub fn current_transform(&self) -> SurfaceTransform {
		self.surface_capabilities.current_transform
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::PhysicalDevice`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::PhysicalDevice> for Device {
	fn unwrap(&self) -> &erupt::vk::PhysicalDevice {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::PhysicalDevice {
		&mut self._internal
	}
}

impl std::fmt::Debug for Device {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.properties)
	}
}

impl std::fmt::Display for Device {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}(v{}) running driver version v{}",
			self.name(),
			self.api_version(),
			self.driver_version()
		)
	}
}

/// Various properties that a physical device needs
/// to have or would be beneficial to have for a given application.
#[derive(Debug, Clone)]
pub enum Constraint {
	/// A set of [`QueueFlags`] (and if a surface is required).
	/// Always required when provided.
	HasQueueFamily(QueueFlags, /*requires_surface*/ bool),
	/// A required pair of [`Format`] and [`ColorSpace`].
	/// Always required when provided.
	HasSurfaceFormats(Format, ColorSpace),
	/// An optional or required constraint on a mode of presentation.
	/// Allows for an optional score. If the score is `None`, the constraint is required.
	CanPresentWith(PresentMode, /*score*/ Option<u32>),
	/// An optional or required constraint that the device be of a specific kind.
	/// Allows for an optional score. If the score is `None`, the constraint is required.
	IsDeviceType(physical::Kind, /*score*/ Option<u32>),
	/// The name of a specific device extension.
	/// Always required when provided.
	HasExtension(String),
	/// A collector of constraints which are applied as a group.
	/// Will check member constraints, until one of them passes. Then stops checking.
	/// If the second param is true, then if none of the constraints need to be successful
	/// for the device to be applicable.
	PrioritizedSet(Vec<Constraint>, /*set_is_optional*/ bool),
}

#[doc(hidden)]
impl Device {
	
	/// Determines if the device can support all the desired rules/properties.
	/// Returns `None` if some constraint failed, otherwise returns the score of the support.
	pub fn score_against_constraints(
		&mut self,
		constraints: &Vec<Constraint>,
		break_on_first_success: bool
	) -> Result<u32, Constraint> {
		let mut total_score = 0;
		for constraint in constraints {
			total_score += self.score_constraint(&constraint)?;
			if break_on_first_success { break; }
		}
		Ok(total_score)
	}

	pub fn score_constraint(&mut self, constraint: &Constraint) -> Result<u32, Constraint> {
		match constraint {
			Constraint::HasQueueFamily(flags, requires_surface) => {
				match self.get_queue_index(*flags, *requires_surface) {
					Some(_queue_family_index) => Ok(0),
					None => Err(constraint.clone()),
				}
			}
			Constraint::HasSurfaceFormats(format, color_space) => {
				if self.contains_all_surface_constraints(*format, *color_space) {
					Ok(0)
				} else {
					Err(constraint.clone())
				}
			}
			Constraint::CanPresentWith(mode, score_or_required) => {
				if self.present_modes.contains(mode) {
					self.selected_present_mode = *mode;
					Ok(match score_or_required {
						Some(score) => *score,
						None => 0,
					})
				} else {
					match score_or_required {
						Some(_) => Ok(0),
						None => Err(constraint.clone()),
					}
				}
			}
			Constraint::IsDeviceType(kind, score_or_required) => {
				if self.device_type() == *kind {
					Ok(match score_or_required {
						Some(score) => *score,
						None => 0,
					})
				} else {
					match score_or_required {
						Some(_) => Ok(0),
						None => Err(constraint.clone()),
					}
				}
			}
			Constraint::HasExtension(ext_name) => {
				let ext_prop = self
					.extension_properties
					.iter()
					.find(|(name, _)| *name == ext_name);
				if ext_prop.is_some() {
					Ok(0)
				} else {
					Err(constraint.clone())
				}
			}
			Constraint::PrioritizedSet(constraint_list, set_is_optional) => {
				match self.score_against_constraints(constraint_list, true) {
					Ok(total_score_for_set) => Ok(total_score_for_set),
					Err(subconstraint) => {
						if *set_is_optional {
							Ok(0)
						} else {
							Err(subconstraint)
						}
					}
				}
			}
		}
	}
}
