use crate::{
	backend, command,
	device::logical,
	utility::{self, VulkanObject},
};
use std::rc::Rc;

pub struct Pool {
	_device: Rc<logical::Device>,
	_internal: backend::vk::CommandPool,
}

impl Pool {
	pub fn create(
		device: &Rc<logical::Device>,
		queue_family_index: usize,
	) -> utility::Result<Pool> {
		let inst = logical::Device::create_command_pool(&device, queue_family_index as u32)?;
		Ok(Pool {
			_device: device.clone(),
			_internal: inst,
		})
	}

	pub fn allocate_buffers(&self, amount: usize) -> utility::Result<Vec<command::Buffer>> {
		logical::Device::allocate_command_buffers(&self._device, &self, amount)
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::CommandPool`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::CommandPool> for Pool {
	fn unwrap(&self) -> &backend::vk::CommandPool {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::CommandPool {
		&mut self._internal
	}
}

impl Drop for Pool {
	fn drop(&mut self) {
		self._device.destroy_command_pool(self._internal)
	}
}
