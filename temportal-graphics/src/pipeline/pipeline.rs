use crate::{backend, device::logical, utility::VulkanObject};

use std::rc::Rc;

/// A vulkan Pipeline. A given pipeline is only valid for a specific [`Render Pass`](crate::renderpass::Pass),
/// and is used to issue commands to the [`GPU`](crate::device::physical::Device).
pub struct Pipeline {
	_device: Rc<logical::Device>,
	_internal: backend::vk::Pipeline,
}

impl Pipeline {
	pub fn from(_device: Rc<logical::Device>, _internal: backend::vk::Pipeline) -> Pipeline {
		Pipeline { _device, _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Pipeline`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::Pipeline> for Pipeline {
	fn unwrap(&self) -> &backend::vk::Pipeline {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Pipeline {
		&mut self._internal
	}
}

impl Drop for Pipeline {
	fn drop(&mut self) {
		self._device.destroy_pipeline(self._internal)
	}
}
