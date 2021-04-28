use crate::{backend, command, device::logical, utility};

use std::rc::Rc;

pub struct Queue {
	queue_family_index: usize,
	_internal: backend::vk::Queue,
	device: Rc<logical::Device>,
}

impl Queue {
	pub fn from(
		device: Rc<logical::Device>,
		_internal: backend::vk::Queue,
		queue_family_index: usize,
	) -> Queue {
		Queue {
			device,
			_internal,
			queue_family_index,
		}
	}

	pub fn index(&self) -> usize {
		self.queue_family_index
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

/// A trait exposing the internal value for the wrapped [`backend::vk::Queue`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<backend::vk::Queue> for Queue {
	fn unwrap(&self) -> &backend::vk::Queue {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Queue {
		&mut self._internal
	}
}
