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

	pub fn begin_label<TStr>(&self, name: TStr, color: [f32; 4])
	where
		TStr: Into<String>,
	{
		self.device.begin_queue_label(&self, name, color);
	}

	pub fn insert_label<TStr>(&self, name: TStr, color: [f32; 4])
	where
		TStr: Into<String>,
	{
		self.device.insert_queue_label(&self, name, color);
	}

	pub fn end_label(&self) {
		self.device.end_queue_label(&self);
	}

	pub fn submit(
		&self,
		infos: Vec<command::SubmitInfo>,
		signal_fence_when_complete: Option<&command::Fence>,
	) -> utility::Result<()> {
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
	#[profiling::function]
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

impl utility::HandledObject for Queue {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Queue as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
