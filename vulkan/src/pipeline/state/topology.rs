use crate::{backend, flags};

pub struct Topology {
	primitive: flags::PrimitiveTopology,
	restart_primitives: bool,
}

impl Default for Topology {
	fn default() -> Self {
		Self {
			primitive: flags::PrimitiveTopology::TRIANGLE_LIST,
			restart_primitives: false,
		}
	}
}

impl Topology {
	pub fn with_primitive(mut self, primitive: flags::PrimitiveTopology) -> Self {
		self.primitive = primitive;
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::PipelineInputAssemblyStateCreateInfo {
		backend::vk::PipelineInputAssemblyStateCreateInfo::builder()
			.topology(self.primitive)
			.primitive_restart_enable(self.restart_primitives)
			.build()
	}
}
