use crate::utility::VulkanObject;
use erupt;

pub struct Image {
	_internal: erupt::vk::Image,
}

impl Image {
	pub fn from(_internal: erupt::vk::Image) -> Image {
		Image { _internal }
	}
}

impl VulkanObject<erupt::vk::Image> for Image {
	fn unwrap(&self) -> &erupt::vk::Image {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::Image {
		&mut self._internal
	}
}
