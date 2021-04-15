use crate::{device::logical, image, utility::VulkanInfo};
use erupt;

pub struct ViewInfo {}

impl ViewInfo {}

impl VulkanInfo<erupt::vk::ImageViewCreateInfo> for ViewInfo {
	fn to_vk(&mut self) -> erupt::vk::ImageViewCreateInfo {
		erupt::vk::ImageViewCreateInfoBuilder::new().build()
	}
}

impl ViewInfo {
	pub fn create_object(&mut self, device: &logical::Device) -> image::View {
		let info = self.to_vk();
		image::View::from(device.create_image_view(info))
	}
}
