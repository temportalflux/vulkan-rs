use crate::{
	backend,
	device::logical,
	flags,
	utility::{self, HandledObject},
};

use std::sync;

/// A short-term reference to a particular syncing object.
pub enum SyncObject<'a> {
	Semaphore(&'a Semaphore),
	Fence(&'a Fence),
}

/// A signal on the GPU that is signaled when a set of submitted commands have completed.
///
/// Used for communicating only within the GPU about the order command buffers should be executed.
pub struct Semaphore {
	internal: backend::vk::Semaphore,
	device: sync::Arc<logical::Device>,
	#[allow(dead_code)]
	name: String,
}

impl Semaphore {
	pub fn new(device: &sync::Arc<logical::Device>, name: &str) -> utility::Result<Semaphore> {
		let info = backend::vk::SemaphoreCreateInfo::builder().build();
		let vk = unsafe { device.create_semaphore(&info, None) }?;
		let semaphore = Semaphore::from(device.clone(), vk, name.to_owned());
		device.set_object_name_logged(&semaphore.create_name(name));
		Ok(semaphore)
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::Semaphore,
		name: String,
	) -> Semaphore {
		Semaphore {
			device,
			internal,
			name,
		}
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
		/*
		log::debug!(
			target: crate::LOG,
			"Dropping Semaphore: {:?}",
			self.name
		);
		*/
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
	#[allow(dead_code)]
	name: String,
}

impl Fence {
	pub fn new(
		device: &sync::Arc<logical::Device>,
		name: &str,
		state: flags::FenceState,
	) -> utility::Result<Fence> {
		let info = backend::vk::FenceCreateInfo::builder().flags(state).build();
		let vk = unsafe { device.create_fence(&info, None) }?;
		let fence = Fence::from(device.clone(), vk, name.to_owned());
		device.set_object_name_logged(&fence.create_name(name));
		Ok(fence)
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::Fence,
		name: String,
	) -> Fence {
		Fence {
			device,
			internal,
			name,
		}
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
		/*
		log::debug!(
			target: crate::LOG,
			"Dropping Fence: {:?}",
			self.name
		);
		*/
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
