use crate::{backend, command, device::logical, utility};

use std::sync;

pub struct Queue {
	queue_family_index: usize,
	internal: backend::vk::Queue,
	device: sync::Arc<logical::Device>,
}

impl Queue {
	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::Queue,
		queue_family_index: usize,
	) -> Queue {
		Queue {
			device,
			internal,
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
		use backend::version::DeviceV1_0;
		let infos = infos
			.iter()
			.map(command::SubmitInfo::as_vk)
			.collect::<Vec<_>>();
		Ok(unsafe {
			self.device.queue_submit(
				self.internal,
				&infos,
				signal_fence_when_complete.map_or(backend::vk::Fence::null(), |obj| **obj),
			)
		}?)
	}
	/// returns true if the swapchain is suboptimal
	pub fn present(&self, info: command::PresentInfo) -> utility::Result</*suboptimal*/ bool> {
		Ok(unsafe {
			self.device
				.unwrap_swapchain()
				.queue_present(self.internal, &info.as_vk())
		}?)
	}
}

impl std::ops::Deref for Queue {
	type Target = backend::vk::Queue;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}
