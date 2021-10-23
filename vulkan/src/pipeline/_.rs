mod builder;
pub use builder::*;

/// Structures for creating a pipeline layout object.
pub mod layout;

mod pipeline;
pub use pipeline::*;

/// Structures around the various properties about a pipeline that are used in the builder.
pub mod state;

#[derive(Clone)]
pub struct PushConstantRange(crate::flags::ShaderKind, usize, usize);
impl Default for PushConstantRange {
	fn default() -> Self {
		Self(crate::flags::ShaderKind::Vertex, 0, 0)
	}
}
impl PushConstantRange {
	pub fn with_stage(mut self, stage: crate::flags::ShaderKind) -> Self {
		self.0 = stage;
		self
	}
	pub fn with_offset(mut self, offset: usize) -> Self {
		self.1 = offset;
		self
	}
	pub fn with_size(mut self, size: usize) -> Self {
		self.2 = size;
		self
	}
}
impl Into<crate::backend::vk::PushConstantRange> for PushConstantRange {
	fn into(self) -> crate::backend::vk::PushConstantRange {
		crate::backend::vk::PushConstantRange::builder()
			.stage_flags(self.0.into())
			.offset(self.1 as u32)
			.size(self.2 as u32)
			.build()
	}
}
