use crate::utility::VulkanObject;
use erupt;

pub struct Surface {
	_internal: erupt::vk::SurfaceKHR,
}

impl Surface {
	pub fn from(_internal: erupt::vk::SurfaceKHR) -> Surface {
		Surface { _internal }
	}
}

impl VulkanObject<erupt::vk::SurfaceKHR> for Surface {
	fn unwrap(&self) -> &erupt::vk::SurfaceKHR {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::SurfaceKHR {
		&mut self._internal
	}
}
