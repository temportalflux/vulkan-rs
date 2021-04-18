use crate::{
	command, flags,
	utility::{VulkanInfo, VulkanObject},
};
use erupt;

/// Data used to submit commands to a [`Queue`](crate::device::logical::Queue).
/// It is NOT safe to keep this struct around for more than 1 stack,
/// as it stores unsafe Vulkan handles/pointers.
pub struct SubmitInfo {
	semaphores_to_wait_for: Vec<erupt::vk::Semaphore>,
	stages_waited_for: Vec<flags::PipelineStage>,
	buffers: Vec<erupt::vk::CommandBuffer>,
	semaphors_to_signal_when_complete: Vec<erupt::vk::Semaphore>,
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
	pub fn wait_for(mut self, semaphore: &command::Semaphore, stage: flags::PipelineStage) -> Self {
		self.semaphores_to_wait_for.push(*semaphore.unwrap());
		self.stages_waited_for.push(stage);
		self
	}

	pub fn add_buffer(mut self, buffer: &command::Buffer) -> Self {
		self.buffers.push(*buffer.unwrap());
		self
	}

	pub fn signal_when_complete(mut self, semaphore: &command::Semaphore) -> Self {
		self.semaphors_to_signal_when_complete
			.push(*semaphore.unwrap());
		self
	}
}

impl VulkanInfo<erupt::vk::SubmitInfo> for SubmitInfo {
	fn to_vk(&self) -> erupt::vk::SubmitInfo {
		erupt::vk::SubmitInfoBuilder::new()
			.wait_semaphores(&self.semaphores_to_wait_for)
			.wait_dst_stage_mask(&self.stages_waited_for)
			.command_buffers(&self.buffers)
			.signal_semaphores(&self.semaphors_to_signal_when_complete)
			.build()
	}
}
