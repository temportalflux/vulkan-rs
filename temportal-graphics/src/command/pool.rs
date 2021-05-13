use crate::{
	backend, command,
	device::logical,
	flags,
	utility::{self, VulkanObject},
};
use std::sync;

pub struct Pool {
	internal: backend::vk::CommandPool,
	device: sync::Arc<logical::Device>,
}

impl Pool {
	pub fn create(
		device: &sync::Arc<logical::Device>,
		queue_family_index: usize,
		flags: Option<flags::CommandPoolCreate>,
	) -> utility::Result<Pool> {
		use backend::version::DeviceV1_0;
		let info = backend::vk::CommandPoolCreateInfo::builder()
			.queue_family_index(queue_family_index as u32)
			.flags(flags.unwrap_or_default())
			.build();
		let internal =
			utility::as_vulkan_error(unsafe { device.unwrap().create_command_pool(&info, None) })?;
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
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.unwrap()
				.destroy_command_pool(self.internal, None)
		};
	}
}
