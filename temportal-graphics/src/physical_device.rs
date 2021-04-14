use super::*;

struct QueueFamily {
	index: usize,
	properties: erupt::vk::QueueFamilyProperties,
	supports_surface: bool,
}

pub struct PhysicalDevice {
	_internal: erupt::vk::PhysicalDevice,
	properties: erupt::vk::PhysicalDeviceProperties,
	queue_families: Vec<QueueFamily>,
	surface_formats: Vec<erupt::vk::SurfaceFormatKHR>,
	present_modes: Vec<erupt::vk::PresentModeKHR>,
	extension_properties: Vec<erupt::vk::ExtensionProperties>,
}

impl PhysicalDevice {
	pub fn new(
		instance: &Instance,
		vk: erupt::vk::PhysicalDevice,
		surface: &erupt::vk::SurfaceKHR,
	) -> PhysicalDevice {
		PhysicalDevice {
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
			extension_properties: instance.enumerate_device_extension_properties(&vk),
		}
	}

	pub fn device_type(&self) -> PhysicalDeviceKind {
		self.properties.device_type
	}

	pub fn name(&self) -> String {
		unsafe { std::ffi::CStr::from_ptr(&self.properties.device_name as *const i8) }
			.to_owned()
			.into_string()
			.unwrap()
	}

	pub fn api_version(&self) -> String {
		as_version_string(&self.properties.api_version)
	}

	pub fn driver_version(&self) -> String {
		as_version_string(&self.properties.driver_version)
	}

	pub fn get_queue_index(&self, flags: QueueFlags, requires_surface: bool) -> Option<usize> {
		match self.queue_families.iter().find(|family| {
			family.properties.queue_flags.contains(flags)
				&& (!requires_surface || family.supports_surface)
		}) {
			Some(family) => Some(family.index),
			None => None,
		}
	}

	pub fn contains_all_surface_constraints(&self, constraints: &SurfaceConstraints) -> bool {
		// the constraints which are not fullfilled after surface_formats is scanned
		let mut leftover_constraints = constraints.clone();
		for supported_format in self.surface_formats.iter() {
			// For each format supported by the device, remove the format from the constraints (if it is in there).
			// Its fine if the format does not exist in the constraints, that just means the device supports more formats than the user requires.
			leftover_constraints
				.formats
				.retain(|fmt| *fmt != supported_format.format);
			// Also remove the supportted color space format for the same reasons
			leftover_constraints
				.color_spaces
				.retain(|fmt| *fmt != supported_format.color_space);
		}
		// The device supports all required constraints if the leftover_constraints are empty
		leftover_constraints.formats.is_empty() && leftover_constraints.color_spaces.is_empty()
	}
}

impl std::fmt::Debug for PhysicalDevice {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.properties)
	}
}

impl std::fmt::Display for PhysicalDevice {
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

#[derive(Clone, Debug)]
pub struct SurfaceConstraints {
	pub formats: Vec<Format>,
	pub color_spaces: Vec<ColorSpace>,
}

#[derive(Debug, Clone)]
pub enum PhysicalDeviceConstraint {
	HasQueueFamily(QueueFlags, /*requires_surface*/ bool),
	HasSurfaceFormats(SurfaceConstraints),
	CanPresentWith(PresentMode, /*score*/ Option<u32>),
	IsDeviceType(PhysicalDeviceKind, /*score*/ Option<u32>),
	PrioritizedSet(Vec<PhysicalDeviceConstraint>, /*set_is_optional*/ bool),
}

impl PhysicalDevice {
	/// Determines if the device can support all the desired rules/properties.
	/// Returns `None` if some constraint failed, otherwise returns the score of the support.
	pub fn score_against_constraints(
		&self,
		constraints: &Vec<PhysicalDeviceConstraint>,
	) -> Result<u32, PhysicalDeviceConstraint> {
		let mut total_score = 0;
		for constraint in constraints {
			total_score += self.score_constraint(&constraint)?;
		}
		Ok(total_score)
	}

	pub fn score_constraint(
		&self,
		constraint: &PhysicalDeviceConstraint,
	) -> Result<u32, PhysicalDeviceConstraint> {
		match constraint {
			PhysicalDeviceConstraint::HasQueueFamily(flags, requires_surface) => {
				match self.get_queue_index(*flags, *requires_surface) {
					Some(_queue_family_index) => Ok(0),
					None => Err(constraint.clone()),
				}
			}
			PhysicalDeviceConstraint::HasSurfaceFormats(format_constraints) => {
				if self.contains_all_surface_constraints(format_constraints) {
					Ok(0)
				} else {
					Err(constraint.clone())
				}
			}
			PhysicalDeviceConstraint::CanPresentWith(mode, score_or_required) => {
				if self.present_modes.contains(mode) {
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
			PhysicalDeviceConstraint::IsDeviceType(kind, score_or_required) => {
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
			PhysicalDeviceConstraint::PrioritizedSet(constraint_list, set_is_optional) => {
				match self.score_against_constraints(constraint_list) {
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
