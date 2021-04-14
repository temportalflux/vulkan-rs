use crate::{
	device::{logical, physical},
	instance::Instance,
	utility::{self, VulkanInfo, VulkanObject},
};
use erupt;

pub struct DeviceQueue {
	pub queue_family_index: usize,
	pub priorities: Vec<f32>,
}

pub struct Info {
	extension_names: Vec<String>,
	layer_names: Vec<String>,

	extension_names_raw: Vec<utility::CStrPtr>,
	layer_names_raw: Vec<utility::CStrPtr>,

	queues: Vec<erupt::vk::DeviceQueueCreateInfo>,
	features: erupt::vk::PhysicalDeviceFeatures,
}

impl Info {
	pub fn new() -> Info {
		Info {
			extension_names: Vec::new(),
			layer_names: Vec::new(),

			extension_names_raw: Vec::new(),
			layer_names_raw: Vec::new(),

			queues: Vec::new(),
			features: erupt::vk::PhysicalDeviceFeatures::default(),
		}
	}

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

	pub fn set_validation_enabled(self, enabled: bool) -> Self {
		if enabled {
			self.add_layer("VK_LAYER_KHRONOS_validation")
		} else {
			self
		}
	}

	pub fn add_queue(mut self, queue: DeviceQueue) -> Self {
		self.queues.push(
				erupt::vk::DeviceQueueCreateInfoBuilder::new()
					.queue_family_index(queue.queue_family_index as u32)
					.queue_priorities(&queue.priorities)
					.build()
		);
		self
	}

	pub fn create_object(
		&mut self,
		instance: &Instance,
		physical_device: &physical::Device,
	) -> logical::Device {
		let info = self.to_vk();
		logical::Device::new(
			erupt::DeviceLoader::new(&instance.unwrap(), *physical_device.unwrap(), &info, None)
				.unwrap(),
		)
	}
}

impl VulkanInfo<erupt::vk::DeviceCreateInfo> for Info {
	fn to_vk(&mut self) -> erupt::vk::DeviceCreateInfo {
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
		
		let mut info = erupt::vk::DeviceCreateInfo::default();

		info.pp_enabled_extension_names = self.extension_names_raw.as_ptr() as _;
		info.enabled_extension_count = self.extension_names_raw.len() as _;

		info.pp_enabled_layer_names = self.layer_names_raw.as_ptr() as _;
		info.enabled_layer_count = self.layer_names_raw.len() as _;

		info.p_queue_create_infos = self.queues.as_ptr() as _;
		info.queue_create_info_count = self.queues.len() as _;

		info.p_enabled_features = &self.features as _;

		info
	}
}
