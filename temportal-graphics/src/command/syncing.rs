use crate::{backend, device::logical, utility};

use std::rc::Rc;

pub struct Semaphore {
	_device: Rc<logical::Device>,
	_internal: backend::vk::Semaphore,
}

impl Semaphore {
	pub fn from(_device: Rc<logical::Device>, _internal: backend::vk::Semaphore) -> Semaphore {
		Semaphore { _device, _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Semaphore`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<backend::vk::Semaphore> for Semaphore {
	fn unwrap(&self) -> &backend::vk::Semaphore {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Semaphore {
		&mut self._internal
	}
}

impl Drop for Semaphore {
	fn drop(&mut self) {
		self._device.destroy_semaphore(self._internal)
	}
}

pub struct Fence {
	_device: Rc<logical::Device>,
	_internal: backend::vk::Fence,
}

impl Fence {
	pub fn from(_device: Rc<logical::Device>, _internal: backend::vk::Fence) -> Fence {
		Fence { _device, _internal }
	}
}

impl Drop for Fence {
	fn drop(&mut self) {
		self._device.destroy_fence(self._internal)
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Fence`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<backend::vk::Fence> for Fence {
	fn unwrap(&self) -> &backend::vk::Fence {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Fence {
		&mut self._internal
	}
}
