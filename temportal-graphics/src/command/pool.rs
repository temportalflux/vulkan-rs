use crate::{
	device::logical,
	utility::{self, VulkanObject},
};
use erupt;

pub struct Pool {
	_internal: erupt::vk::CommandPool,
}

impl Pool {
	pub fn create(device: &logical::Device, queue_family_index: usize) -> utility::Result<Pool> {
		let inst = device.create_command_pool(queue_family_index as u32)?;
		Ok(Pool { _internal: inst })
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
