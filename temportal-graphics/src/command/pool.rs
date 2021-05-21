use crate::{
	backend, command,
	device::logical,
	flags,
	utility::{self},
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
		let internal = unsafe { device.create_command_pool(&info, None) }?;
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
			.command_pool(**self)
			.level(level)
			.command_buffer_count(amount as u32)
			.build();
		Ok(unsafe { self.device.allocate_command_buffers(&info) }?
			.into_iter()
			.map(|vk_buffer| command::Buffer::from(self.device.clone(), vk_buffer))
			.collect::<Vec<_>>())
	}

	pub fn free_buffers(&self, buffers: Vec<command::Buffer>) {
		use backend::version::DeviceV1_0;
		let vk_buffers = buffers.iter().map(|cmd_buf| **cmd_buf).collect::<Vec<_>>();
		unsafe {
			self.device
				.free_command_buffers(self.internal, &vk_buffers[..])
		};
	}
}

impl std::ops::Deref for Pool {
	type Target = backend::vk::CommandPool;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Pool {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.destroy_command_pool(self.internal, None) };
	}
}
