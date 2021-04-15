use crate::utility::VulkanObject;
use erupt;

pub struct Pipeline {
	_internal: erupt::vk::Pipeline,
}

impl Pipeline {
	pub fn from(_internal: erupt::vk::Pipeline) -> Pipeline {
		Pipeline { _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::Pipeline`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::Pipeline> for Pipeline {
	fn unwrap(&self) -> &erupt::vk::Pipeline {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::Pipeline {
		&mut self._internal
	}
}
