use crate::{backend, command, flags};
use std::sync;

/// Data used to submit commands to a [`Queue`](crate::device::logical::Queue).
///
/// It is NOT safe to keep this struct around for more than 1 stack,
/// as it stores unsafe Vulkan handles/pointers.
pub struct SubmitInfo {
	semaphores_to_wait_for: Vec<backend::vk::Semaphore>,
	stages_waited_for: Vec<backend::vk::PipelineStageFlags>,
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
	/// Adds a collection of signals on the GPU that the command buffer should wait
	/// for before executing the commands in the buffer being presented.
	pub fn wait_for_semaphores(mut self, semaphores: &Vec<sync::Arc<command::Semaphore>>) -> Self {
		for rc in semaphores {
			self.semaphores_to_wait_for.push(***rc);
		}
		self
	}

	/// Adds a signal on the GPU that the command buffer should wait
	/// for before executing the commands in the buffer being presented.
	pub fn wait_for(mut self, semaphore: &command::Semaphore, stage: flags::PipelineStage) -> Self {
		self.semaphores_to_wait_for.push(**semaphore);
		self.stages_waited_for.push(stage.into());
		self
	}

	/// Adds a command buffer that should be submitted to the GPU for execution.
	pub fn add_buffer(mut self, buffer: &command::Buffer) -> Self {
		self.buffers.push(**buffer);
		self
	}

	/// Adds a GPU signal that should be marked as "signaled" when the command buffer has been executed.
	pub fn signal_when_complete(mut self, semaphore: &command::Semaphore) -> Self {
		self.semaphors_to_signal_when_complete.push(**semaphore);
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::SubmitInfo {
		backend::vk::SubmitInfo::builder()
			.wait_semaphores(&self.semaphores_to_wait_for)
			.wait_dst_stage_mask(&self.stages_waited_for[..])
			.command_buffers(&self.buffers)
			.signal_semaphores(&self.semaphors_to_signal_when_complete)
			.build()
	}
}
