use crate::{device::logical, utility};
use erupt;
use std::rc::Rc;

pub struct Semaphore {
	_device: Rc<logical::Device>,
	_internal: erupt::vk::Semaphore,
}

impl Semaphore {
	pub fn from(_device: Rc<logical::Device>, _internal: erupt::vk::Semaphore) -> Semaphore {
		Semaphore { _device, _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::Semaphore`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<erupt::vk::Semaphore> for Semaphore {
	fn unwrap(&self) -> &erupt::vk::Semaphore {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::Semaphore {
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
	_internal: erupt::vk::Fence,
}

impl Fence {
	pub fn from(_device: Rc<logical::Device>, _internal: erupt::vk::Fence) -> Fence {
		Fence { _device, _internal }
	}

	pub fn is_valid(&self) -> bool {
		!self._internal.is_null()
	}
}

impl Drop for Fence {
	fn drop(&mut self) {
		self._device.destroy_fence(self._internal)
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::Fence`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<erupt::vk::Fence> for Fence {
	fn unwrap(&self) -> &erupt::vk::Fence {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::Fence {
		&mut self._internal
	}
}
