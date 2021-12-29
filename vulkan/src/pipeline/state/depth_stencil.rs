use crate::{
	backend,
	flags::{CompareOp, StencilOp},
};

#[derive(Default, Clone, Copy)]
pub struct StencilOpState {
	fail_op: StencilOp,
	pass_op: StencilOp,
	depth_fail_op: StencilOp,
	compare_op: CompareOp,
	compare_mask: u32,
	write_mask: u32,
	reference: u32,
}

impl StencilOpState {
	pub fn with_fail_op(mut self, op: StencilOp) -> Self {
		self.fail_op = op;
		self
	}

	pub fn with_pass_op(mut self, op: StencilOp) -> Self {
		self.pass_op = op;
		self
	}

	pub fn with_depth_fail_op(mut self, op: StencilOp) -> Self {
		self.depth_fail_op = op;
		self
	}

	pub fn with_compare_op(mut self, op: CompareOp) -> Self {
		self.compare_op = op;
		self
	}

	pub fn with_compare_mask(mut self, mask: u32) -> Self {
		self.compare_mask = mask;
		self
	}

	pub fn with_write_mask(mut self, mask: u32) -> Self {
		self.write_mask = mask;
		self
	}

	pub fn with_reference(mut self, reference: u32) -> Self {
		self.reference = reference;
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::StencilOpState {
		backend::vk::StencilOpState::builder()
			.fail_op(self.fail_op)
			.pass_op(self.pass_op)
			.depth_fail_op(self.depth_fail_op)
			.compare_op(self.compare_op)
			.compare_mask(self.compare_mask)
			.write_mask(self.write_mask)
			.reference(self.reference)
			.build()
	}
}

#[derive(Default, Clone, Copy)]
pub struct DepthStencil {
	depth_test_enable: bool,
	depth_write_enable: bool,
	depth_compare_op: CompareOp,
	depth_bounds_test_enable: bool,
	depth_bounds: (f32, f32),
	stencil_test_enable: bool,
	stencil_front: StencilOpState,
	stencil_back: StencilOpState,
}

impl DepthStencil {
	pub fn with_depth_test(mut self) -> Self {
		self.depth_test_enable = true;
		self
	}

	pub fn with_depth_write(mut self) -> Self {
		self.depth_write_enable = true;
		self
	}

	pub fn with_depth_compare_op(mut self, op: CompareOp) -> Self {
		self.depth_compare_op = op;
		self
	}

	pub fn with_depth_bounds_test(mut self) -> Self {
		self.depth_bounds_test_enable = true;
		self
	}

	pub fn with_stencil_test(mut self) -> Self {
		self.stencil_test_enable = true;
		self
	}

	pub fn with_stencil_front(mut self, state: StencilOpState) -> Self {
		self.stencil_front = state;
		self
	}

	pub fn with_stencil_back(mut self, state: StencilOpState) -> Self {
		self.stencil_back = state;
		self
	}

	pub fn with_depth_bounds(mut self, min: f32, max: f32) -> Self {
		self.depth_bounds = (min, max);
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::PipelineDepthStencilStateCreateInfo {
		backend::vk::PipelineDepthStencilStateCreateInfo::builder()
			.depth_test_enable(self.depth_test_enable)
			.depth_write_enable(self.depth_write_enable)
			.depth_compare_op(self.depth_compare_op)
			.depth_bounds_test_enable(self.depth_bounds_test_enable)
			.min_depth_bounds(self.depth_bounds.0)
			.max_depth_bounds(self.depth_bounds.1)
			.stencil_test_enable(self.stencil_test_enable)
			.front(self.stencil_front.as_vk())
			.back(self.stencil_back.as_vk())
			.build()
	}
}
