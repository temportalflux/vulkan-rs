use crate::{
	backend,
	device::logical,
	flags,
	utility::{self, HandledObject},
};

use std::sync;

/// A signal on the GPU that is signaled when a set of submitted commands have completed.
///
/// Used for communicating only within the GPU about the order command buffers should be executed.
pub struct Semaphore {
	internal: backend::vk::Semaphore,
	device: sync::Arc<logical::Device>,
}

impl Semaphore {
	pub fn new(
		device: &sync::Arc<logical::Device>,
		name: Option<String>,
	) -> utility::Result<Semaphore> {
		use backend::version::DeviceV1_0;
		let info = backend::vk::SemaphoreCreateInfo::builder().build();
		let vk = unsafe { device.create_semaphore(&info, None) }?;
		let semaphore = Semaphore::from(device.clone(), vk);
		if let Some(name) = name {
			device.set_object_name_logged(&semaphore.create_name(name.as_str()));
		}
		Ok(semaphore)
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::Semaphore,
	) -> Semaphore {
		Semaphore { device, internal }
	}
}

impl std::ops::Deref for Semaphore {
	type Target = backend::vk::Semaphore;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Semaphore {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.destroy_semaphore(self.internal, None) };
	}
}

impl HandledObject for Semaphore {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Semaphore as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}

/// A signal on the CPU that the GPU marks as signaled when a set of submitted commands have completed.
///
/// Used for communicating from GPU to CPU.
pub struct Fence {
	internal: backend::vk::Fence,
	device: sync::Arc<logical::Device>,
}

impl Fence {
	pub fn new(
		device: &sync::Arc<logical::Device>,
		name: Option<String>,
		state: flags::FenceState,
	) -> utility::Result<Fence> {
		use backend::version::DeviceV1_0;
		let info = backend::vk::FenceCreateInfo::builder().flags(state).build();
		let vk = unsafe { device.create_fence(&info, None) }?;
		let fence = Fence::from(device.clone(), vk);
		if let Some(name) = name {
			device.set_object_name_logged(&fence.create_name(name.as_str()));
		}
		Ok(fence)
	}

	pub(crate) fn from(device: sync::Arc<logical::Device>, internal: backend::vk::Fence) -> Fence {
		Fence { device, internal }
	}
}

impl std::ops::Deref for Fence {
	type Target = backend::vk::Fence;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Fence {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.destroy_fence(self.internal, None) };
	}
}

impl utility::HandledObject for Fence {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Fence as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
