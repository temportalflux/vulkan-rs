use crate::{backend, descriptor::layout::Builder, device::logical, utility::VulkanObject};
use std::rc::Rc;

pub struct SetLayout {
	internal: backend::vk::DescriptorSetLayout,
	device: Rc<logical::Device>,
}

impl SetLayout {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub fn from(
		device: Rc<logical::Device>,
		internal: backend::vk::DescriptorSetLayout,
	) -> SetLayout {
		SetLayout { device, internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::DescriptorSetLayout`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::DescriptorSetLayout> for SetLayout {
	fn unwrap(&self) -> &backend::vk::DescriptorSetLayout {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::DescriptorSetLayout {
		&mut self.internal
	}
}

impl Drop for SetLayout {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.unwrap()
				.destroy_descriptor_set_layout(self.internal, None);
		}
	}
}
