use crate::{
	backend, command,
	device::logical,
	utility::{self, VulkanInfo, VulkanObject},
};

use std::sync;

pub struct Queue {
	queue_family_index: usize,
	internal: backend::vk::Queue,
	device: sync::Arc<logical::Device>,
}

impl Queue {
	pub fn from(
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
		let infos = infos.iter().map(|info| info.to_vk()).collect::<Vec<_>>();
		utility::as_vulkan_error(unsafe {
			self.device.unwrap().queue_submit(
				self.internal,
				&infos,
				signal_fence_when_complete.map_or(backend::vk::Fence::null(), |obj| *obj.unwrap()),
			)
		})
	}
	/// returns true if the swapchain is suboptimal
	pub fn present(&self, info: command::PresentInfo) -> utility::Result</*suboptimal*/ bool> {
		let vk_info = info.to_vk();
		utility::as_vulkan_error(unsafe {
			self.device
				.unwrap_swapchain()
				.queue_present(self.internal, &vk_info)
		})
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Queue`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::Queue> for Queue {
	fn unwrap(&self) -> &backend::vk::Queue {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Queue {
		&mut self.internal
	}
}
