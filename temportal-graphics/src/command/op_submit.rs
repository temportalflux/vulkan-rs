use crate::{backend, command, flags, utility::VulkanInfo};
use std::sync;

/// Data used to submit commands to a [`Queue`](crate::device::logical::Queue).
/// It is NOT safe to keep this struct around for more than 1 stack,
/// as it stores unsafe Vulkan handles/pointers.
pub struct SubmitInfo {
	semaphores_to_wait_for: Vec<backend::vk::Semaphore>,
	stages_waited_for: Vec<flags::PipelineStage>,
	buffers: Vec<backend::vk::CommandBuffer>,
	semaphors_to_signal_when_complete: Vec<backend::vk::Semaphore>,
}

impl Default for SubmitInfo {
	fn default() -> SubmitInfo {
		SubmitInfo {
			semaphores_to_wait_for: Vec::new(),
			stages_waited_for: Vec::new(),
			buffers: Vec::new(),
			semaphors_to_signal_when_complete: Vec::new(),
		}
	}
}

impl SubmitInfo {
	pub fn wait_for_semaphores(mut self, semaphores: &Vec<sync::Arc<command::Semaphore>>) -> Self {
		for rc in semaphores {
			self.semaphores_to_wait_for.push(***rc);
		}
		self
	}

	pub fn wait_for(mut self, semaphore: &command::Semaphore, stage: flags::PipelineStage) -> Self {
		self.semaphores_to_wait_for.push(**semaphore);
		self.stages_waited_for.push(stage);
		self
	}

	pub fn add_buffer(mut self, buffer: &command::Buffer) -> Self {
		self.buffers.push(**buffer);
		self
	}

	pub fn signal_when_complete(mut self, semaphore: &command::Semaphore) -> Self {
		self.semaphors_to_signal_when_complete.push(**semaphore);
		self
	}
}

impl VulkanInfo<backend::vk::SubmitInfo> for SubmitInfo {
	fn to_vk(&self) -> backend::vk::SubmitInfo {
		backend::vk::SubmitInfo::builder()
			.wait_semaphores(&self.semaphores_to_wait_for)
			.wait_dst_stage_mask(&self.stages_waited_for)
			.command_buffers(&self.buffers)
			.signal_semaphores(&self.semaphors_to_signal_when_complete)
			.build()
	}
}
