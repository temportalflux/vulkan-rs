use crate::{
	backend, command,
	device::logical,
	flags,
	utility::{self, VulkanObject},
};
use std::rc::Rc;

pub struct Pool {
	device: Rc<logical::Device>,
	internal: backend::vk::CommandPool,
}

impl Pool {
	pub fn create(
		device: &Rc<logical::Device>,
		queue_family_index: usize,
	) -> utility::Result<Pool> {
		let internal = logical::Device::create_command_pool(&device, queue_family_index as u32)?;
		Ok(Pool {
			device: device.clone(),
			internal,
		})
	}

	pub fn allocate_buffers(
		&self,
		amount: usize,
		level: flags::CommandBufferLevel,
	) -> utility::Result<Vec<command::Buffer>> {
		use backend::version::DeviceV1_0;
		let info = backend::vk::CommandBufferAllocateInfo::builder()
			.command_pool(*self.unwrap())
			.level(level)
			.command_buffer_count(amount as u32)
			.build();
		let alloc_result = utility::as_vulkan_error(unsafe {
			self.device.unwrap().allocate_command_buffers(&info)
		});
		Ok(alloc_result?
			.into_iter()
			.map(|vk_buffer| command::Buffer::from(self.device.clone(), vk_buffer))
			.collect::<Vec<_>>())
	}

	pub fn free_buffers(&self, buffers: Vec<command::Buffer>) {
		use backend::version::DeviceV1_0;
		let vk_buffers = buffers
			.iter()
			.map(|cmd_buf| *cmd_buf.unwrap())
			.collect::<Vec<_>>();
		unsafe {
			self.device
				.unwrap()
				.free_command_buffers(self.internal, &vk_buffers[..])
		};
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::CommandPool`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::CommandPool> for Pool {
	fn unwrap(&self) -> &backend::vk::CommandPool {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::CommandPool {
		&mut self.internal
	}
}

impl Drop for Pool {
	fn drop(&mut self) {
		self.device.destroy_command_pool(self.internal)
	}
}
