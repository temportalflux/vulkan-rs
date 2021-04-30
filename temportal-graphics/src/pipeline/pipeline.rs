use crate::{backend, device::logical, utility::VulkanObject};

use std::rc::Rc;

/// A vulkan Pipeline. A given pipeline is only valid for a specific [`Render Pass`](crate::renderpass::Pass),
/// and is used to issue commands to the [`GPU`](crate::device::physical::Device).
pub struct Pipeline {
	internal: backend::vk::Pipeline,
	device: Rc<logical::Device>,
}

impl Pipeline {
	pub fn from(device: Rc<logical::Device>, internal: backend::vk::Pipeline) -> Pipeline {
		Pipeline { device, internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Pipeline`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::Pipeline> for Pipeline {
	fn unwrap(&self) -> &backend::vk::Pipeline {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Pipeline {
		&mut self.internal
	}
}

impl Drop for Pipeline {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.unwrap().destroy_pipeline(self.internal, None) };
	}
}
