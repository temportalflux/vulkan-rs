use crate::{
	backend,
	device::physical,
	flags::{format::Format, ColorSpace, PresentMode, QueueFlags, SurfaceTransform},
	instance::Instance,
	structs::Extent2D,
	utility, Surface,
};
use std::collections::hash_map::HashMap;
use std::sync;

pub use backend::vk::PhysicalDeviceType as Kind;

struct QueueFamily {
	index: usize,
	properties: backend::vk::QueueFamilyProperties,
	supports_surface: bool,
}

pub struct SurfaceSupport {
	surface_capabilities: backend::vk::SurfaceCapabilitiesKHR,
	surface_formats: Vec<backend::vk::SurfaceFormatKHR>,
	present_modes: Vec<PresentMode>,
}

impl SurfaceSupport {
	/// Returns a range representing the minimum and maximum number of images that the surface can support for a [`Swapchain`](crate::device::swapchain::Swapchain).
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

/// The wrapper for [`Vulkan PhysicalDevice`](backend::vk::PhysicalDevice) objects.
/// Represents a literal GPU.
pub struct Device {
	pub selected_present_mode: PresentMode,

	properties: backend::vk::PhysicalDeviceProperties,
	queue_families: Vec<QueueFamily>,
	extension_properties: HashMap<String, backend::vk::ExtensionProperties>,

	_internal: backend::vk::PhysicalDevice,
	surface: sync::Weak<Surface>,
	instance: sync::Weak<Instance>,
}

impl Device {
	/// The internal constructor. Users should use [`Instance.find_physical_device`](crate::instance::Instance::find_physical_device) to create a vulkan instance.
	pub(crate) fn from(
		instance: &sync::Arc<Instance>,
		vk: backend::vk::PhysicalDevice,
		surface: &sync::Arc<Surface>,
	) -> Device {
		Device {
			instance: sync::Arc::downgrade(&instance),
			surface: sync::Arc::downgrade(&surface),
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
						.does_physical_device_surface_support_khr(&vk, index, surface),
				})
				.collect(),
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
			selected_present_mode: PresentMode::FIFO,
		}
	}

	/// Returns the kind of graphics processor unit this is (i.e. dedicated, integrated, etc).
	pub fn device_type(&self) -> physical::Kind {
		self.properties.device_type
	}

	pub fn max_sampler_anisotropy(&self) -> f32 {
		self.properties.limits.max_sampler_anisotropy
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

	pub fn query_surface_support(&self) -> SurfaceSupport {
		let instance = self.instance.upgrade().unwrap();
		let surface = self.surface.upgrade().unwrap();
		SurfaceSupport {
			surface_capabilities: instance
				.get_physical_device_surface_capabilities(&self._internal, &*surface),
			surface_formats: instance
				.get_physical_device_surface_formats(&self._internal, &*surface),
			present_modes: instance
				.get_physical_device_surface_present_modes(&self._internal, &*surface),
		}
	}
}

impl std::ops::Deref for Device {
	type Target = backend::vk::PhysicalDevice;
	fn deref(&self) -> &Self::Target {
		&self._internal
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

pub fn default_constraints() -> Vec<Constraint> {
	use physical::Constraint::*;
	vec![
		HasSurfaceFormats(Format::B8G8R8A8_SRGB, ColorSpace::SRGB_NONLINEAR),
		HasExtension(String::from("VK_KHR_swapchain")),
		PrioritizedSet(
			vec![
				CanPresentWith(PresentMode::MAILBOX, Some(1)),
				CanPresentWith(PresentMode::FIFO, None),
			],
			false,
		),
		PrioritizedSet(
			vec![
				IsDeviceType(physical::Kind::DISCRETE_GPU, Some(100)),
				IsDeviceType(physical::Kind::INTEGRATED_GPU, Some(0)),
			],
			false,
		),
	]
}

#[doc(hidden)]
impl Device {
	/// Determines if the device can support all the desired rules/properties.
	/// Returns `None` if some constraint failed, otherwise returns the score of the support.
	pub fn score_against_constraints(
		&mut self,
		constraints: &Vec<Constraint>,
		break_on_first_success: bool,
	) -> Result<u32, Constraint> {
		let surface_support = self.query_surface_support();
		let mut total_score = 0;
		for constraint in constraints {
			total_score += self.score_constraint(&constraint, &surface_support)?;
			if break_on_first_success {
				break;
			}
		}
		Ok(total_score)
	}

	pub fn score_constraint(
		&mut self,
		constraint: &Constraint,
		surface_support: &SurfaceSupport,
	) -> Result<u32, Constraint> {
		match constraint {
			Constraint::HasQueueFamily(flags, requires_surface) => {
				match self.get_queue_index(*flags, *requires_surface) {
					Some(_queue_family_index) => Ok(0),
					None => Err(constraint.clone()),
				}
			}
			Constraint::HasSurfaceFormats(format, color_space) => {
				for supported_format in surface_support.surface_formats.iter() {
					if supported_format.format == *format
						&& supported_format.color_space == *color_space
					{
						return Ok(0);
					}
				}
				Err(constraint.clone())
			}
			Constraint::CanPresentWith(mode, score_or_required) => {
				if surface_support.present_modes.contains(mode) {
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
