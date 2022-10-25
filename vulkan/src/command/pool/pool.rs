use crate::{
	backend, command,
	device::logical,
	flags,
	utility::{self},
};
use std::sync;

/// Command pools are opaque objects that command buffer memory is allocated from,
/// and which allow the implementation to amortize the cost of resource creation across multiple command buffers.
///
/// Command pools are externally synchronized, meaning that a command pool must not be used concurrently in multiple threads.
/// That includes use via recording commands on any command buffers allocated from the pool, as well as operations that
/// allocate, free, and reset command buffers or the pool itself.
///
/// Equivalent to [VkCommandPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPool.html).
pub struct Pool {
	name: String,
	internal: backend::vk::CommandPool,
	device: sync::Arc<logical::Device>,
}

impl Pool {
	pub fn builder() -> command::PoolBuilder {
		command::PoolBuilder::default()
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		name: String,
		internal: backend::vk::CommandPool,
	) -> Self {
		Self {
			name,
			internal,
			device,
		}
	}

	/// Creates some amount of [`command buffers`](command::Buffer) at a given level.
	pub fn allocate_buffers(
		&self,
		amount: usize,
		level: flags::CommandBufferLevel,
	) -> utility::Result<Vec<command::Buffer>> {
		self.allocate_named_buffers(
			(0..amount)
				.map(|i| format!("{}.Buffer{}", self.name, i))
				.collect::<Vec<_>>(),
			level,
		)
	}

	/// Creates some amount of [`command buffers`](command::Buffer) at a given level.
	pub fn allocate_named_buffers(
		&self,
		buffer_names: Vec<String>,
		level: flags::CommandBufferLevel,
	) -> utility::Result<Vec<command::Buffer>> {
		use utility::HandledObject;
		let info = backend::vk::CommandBufferAllocateInfo::builder()
			.command_pool(**self)
			.level(level)
			.command_buffer_count(buffer_names.len() as u32)
			.build();
		Ok(unsafe { self.device.allocate_command_buffers(&info) }?
			.into_iter()
			.zip(buffer_names.iter())
			.map(|(vk_buffer, buffer_name)| {
				let buffer =
					command::Buffer::from(self.device.clone(), buffer_name.clone(), vk_buffer);
				self.device
					.set_object_name_logged(&buffer.create_name(buffer_name));
				buffer
			})
			.collect::<Vec<_>>())
	}

	/// Destroys buffers created by the pool.
	///
	/// Use with caution, as the buffers being destroyed will not longer be accessible
	/// (which is why this function consumes ownership of the buffers).
	pub fn free_buffers(&self, buffers: Vec<command::Buffer>) {
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
		unsafe { self.device.destroy_command_pool(self.internal, None) };
	}
}

impl utility::HandledObject for Pool {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::CommandPool as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
