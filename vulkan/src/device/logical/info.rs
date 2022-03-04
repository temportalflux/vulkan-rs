use crate::{
	backend,
	device::{logical, physical},
	instance::Instance,
	utility::{self, HandledObject},
};
use std::sync;

#[derive(Debug)]
pub struct DeviceQueue {
	pub queue_family_index: usize,
	pub priorities: Vec<f32>,
}

/// Collects together information about a [`logical::Device`] that is used by the hardware
/// to construct the logical device to send commands to the hardware.
pub struct Info {
	extension_names: Vec<String>,
	layer_names: Vec<String>,

	extension_names_raw: Vec<utility::CStrPtr>,
	layer_names_raw: Vec<utility::CStrPtr>,

	queues: Vec<DeviceQueue>,
	features: backend::vk::PhysicalDeviceFeatures,

	object_name: Option<String>,
}

impl Default for Info {
	fn default() -> Info {
		Info {
			extension_names: Vec::new(),
			layer_names: Vec::new(),

			extension_names_raw: Vec::new(),
			layer_names_raw: Vec::new(),

			queues: Vec::new(),
			features: backend::vk::PhysicalDeviceFeatures::builder()
				.sampler_anisotropy(true)
				.sample_rate_shading(true)
				.build(),

			object_name: None,
		}
	}
}

impl Info {
	/// Adds the name of a device extension that is required for the logical device.
	/// Users should ensure that this name is present in the constraints passed to
	/// [`Instance.find_physical_device`](crate::instance::Instance::find_physical_device).
	pub fn add_extension(mut self, name: &str) -> Self {
		self.extension_names.push(
			std::ffi::CString::new(name.as_bytes())
				.unwrap()
				.into_string()
				.unwrap(),
		);
		self
	}

	pub fn add_layer(mut self, name: &str) -> Self {
		self.layer_names.push(
			std::ffi::CString::new(name.as_bytes())
				.unwrap()
				.into_string()
				.unwrap(),
		);
		self
	}

	/// Marks that validation is enabled or disabled for the logical device.
	pub fn set_validation_enabled(self, enabled: bool) -> Self {
		if enabled {
			self.add_layer("VK_LAYER_KHRONOS_validation")
		} else {
			self
		}
	}

	/// Ensures that the created device contains a given queue family so transfer queues can be created for it.
	pub fn add_queue(mut self, queue: DeviceQueue) -> Self {
		self.queues.push(queue);
		self
	}

	pub fn with_name<T>(mut self, name: T) -> Self
	where
		T: Into<String>,
	{
		self.object_name = Some(name.into());
		self
	}

	/// Creates the [`Logical Device`](logical::Device) vulkan object using the provided information.
	/// Consumes the info object data.
	pub fn create_object(
		&mut self,
		instance: &sync::Arc<Instance>,
		physical_device: &physical::Device,
	) -> utility::Result<logical::Device> {
		use backend::version::InstanceV1_0;
		self.extension_names_raw = self
			.extension_names
			.iter()
			.map(|owned| utility::to_cstr_ptr(&owned))
			.collect();
		self.layer_names_raw = self
			.layer_names
			.iter()
			.map(|owned| utility::to_cstr_ptr(&owned))
			.collect();
		let queues = self
			.queues
			.iter()
			.map(|queue| {
				backend::vk::DeviceQueueCreateInfo::builder()
					.queue_family_index(queue.queue_family_index as u32)
					.queue_priorities(&queue.priorities)
					.build()
			})
			.collect::<Vec<_>>();

		let mut info = backend::vk::DeviceCreateInfo::default();

		info.pp_enabled_extension_names = self.extension_names_raw.as_ptr() as _;
		info.enabled_extension_count = self.extension_names_raw.len() as _;

		info.pp_enabled_layer_names = self.layer_names_raw.as_ptr() as _;
		info.enabled_layer_count = self.layer_names_raw.len() as _;

		info.p_queue_create_infos = queues.as_ptr() as _;
		info.queue_create_info_count = queues.len() as _;

		info.p_enabled_features = &self.features as _;

		let internal = unsafe { instance.create_device(**physical_device, &info, None) }?;
		let device = logical::Device::from(&instance, internal);
		if let Some(name_ref) = self.object_name.as_ref() {
			device.set_object_name_logged(&device.create_name(name_ref.as_str()));
		}
		Ok(device)
	}
}
