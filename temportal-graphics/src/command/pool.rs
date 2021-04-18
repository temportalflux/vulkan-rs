use crate::{
	device::logical,
	utility::{self, VulkanObject},
};
use erupt;
use std::rc::Rc;

pub struct Pool {
	_device: Rc<logical::Device>,
	_internal: erupt::vk::CommandPool,
}

impl Pool {
	pub fn create(device: Rc<logical::Device>, queue_family_index: usize) -> utility::Result<Pool> {
		let inst = logical::Device::create_command_pool(&device, queue_family_index as u32)?;
		Ok(Pool {
			_device: device,
			_internal: inst,
		})
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::CommandPool`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::CommandPool> for Pool {
	fn unwrap(&self) -> &erupt::vk::CommandPool {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::CommandPool {
		&mut self._internal
	}
}

impl Drop for Pool {
	fn drop(&mut self) {
		self._device.destroy_command_pool(self._internal)
	}
}
