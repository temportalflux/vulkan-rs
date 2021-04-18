use crate::utility;
use erupt;

pub struct Semaphore {
	_internal: erupt::vk::Semaphore,
}

impl Semaphore {
	pub fn from(_internal: erupt::vk::Semaphore) -> Semaphore {
		Semaphore { _internal }
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

pub struct Fence {
	_internal: erupt::vk::Fence,
}

impl Fence {
	pub fn from(_internal: erupt::vk::Fence) -> Fence {
		Fence { _internal }
	}

	pub fn empty() -> Fence {
		Fence::from(erupt::vk::Fence::null())
	}

	pub fn is_valid(&self) -> bool {
		!self._internal.is_null()
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
