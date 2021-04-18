use crate::{command, device::logical, utility};
use erupt;
use std::rc::Rc;

pub struct Queue {
	device: Rc<logical::Device>,
	_internal: erupt::vk::Queue,
}

impl Queue {
	pub fn from(device: Rc<logical::Device>, _internal: erupt::vk::Queue) -> Queue {
		Queue { device, _internal }
	}

	pub fn submit(
		&self,
		infos: Vec<command::SubmitInfo>,
		signal_fence_when_complete: Option<&command::Fence>,
	) -> utility::Result<()> {
		self.device.submit(&self, infos, signal_fence_when_complete)
	}

	pub fn present(&self, info: command::PresentInfo) -> utility::Result<()> {
		self.device.present(&self, info)
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
