use crate::{backend, device::logical, utility::VulkanObject};
use std::rc::Rc;

pub struct Pool {
	internal: backend::vk::DescriptorPool,
	device: Rc<logical::Device>,
}

impl Pool {
	pub fn from(device: Rc<logical::Device>, internal: backend::vk::DescriptorPool) -> Pool {
		Pool { device, internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::DescriptorPool`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::DescriptorPool> for Pool {
	fn unwrap(&self) -> &backend::vk::DescriptorPool {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::DescriptorPool {
		&mut self.internal
	}
}

impl Drop for Pool {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.unwrap()
				.destroy_descriptor_pool(self.internal, None);
		}
	}
}
