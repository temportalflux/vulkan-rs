use crate::{backend, flags};

/// Information about what states can be set dynamically
/// via a [`command buffer`](crate::command::Buffer)
/// for a given [`Pipeline`](crate::pipeline::Pipeline).
pub struct DynamicState {
	states: Vec<flags::DynamicState>,
}

impl Default for DynamicState {
	fn default() -> Self {
		Self { states: Vec::new() }
	}
}

impl DynamicState {
	/// Adds the provided state flag to the list of states that will be set dynamically via a command buffer.
	pub fn with(mut self, state: flags::DynamicState) -> Self {
		self.states.push(state);
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::PipelineDynamicStateCreateInfo {
		backend::vk::PipelineDynamicStateCreateInfo::builder()
			.dynamic_states(&self.states[..])
			.build()
	}
}
