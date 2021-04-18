use crate::utility;
use erupt;

pub struct Queue {
	_internal: erupt::vk::Queue,
}

impl Queue {
	pub fn from(_internal: erupt::vk::Queue) -> Queue {
		Queue { _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::Queue`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<erupt::vk::Queue> for Queue {
	fn unwrap(&self) -> &erupt::vk::Queue {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::Queue {
		&mut self._internal
	}
}
