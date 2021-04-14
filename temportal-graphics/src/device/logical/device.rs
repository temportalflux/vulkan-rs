use crate::utility::VulkanObject;
use erupt;

pub struct Device {
	_internal: erupt::DeviceLoader,
}

impl Device {
	pub fn new(internal: erupt::DeviceLoader) -> Device {
		Device {
			_internal: internal,
		}
	}
}

impl VulkanObject<erupt::DeviceLoader> for Device {
	fn unwrap(&self) -> &erupt::DeviceLoader {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::DeviceLoader {
		&mut self._internal
	}
}
