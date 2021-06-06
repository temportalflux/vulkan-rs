use crate::{
	backend,
	flags::{CullMode, FrontFace, PolygonMode},
};

/// Bias-information when dealing with depth in a [`Pipeline`](crate::pipeline::Pipeline).
#[derive(Clone)]
pub struct DepthBias {
	constant_factor: f32,
	clamp: f32,
	slope_factor: f32,
}

/// Information about the rasterization of fragments during the execution of a [`Pipeline`](crate::pipeline::Pipeline).
#[derive(Clone)]
pub struct Rasterization {
	depth_clamp_enabled: bool,
	depth_bias: Option<DepthBias>,
	rasterizer_discard_enabled: bool,
	polygon_mode: PolygonMode,
	line_width: f32,
	cull_mode: CullMode,
	front_face: FrontFace,
}

impl Default for Rasterization {
	fn default() -> Self {
		Self {
			depth_clamp_enabled: false,
			depth_bias: None,
			rasterizer_discard_enabled: false,
			polygon_mode: PolygonMode::FILL,
			line_width: 1.0,
			cull_mode: CullMode::BACK,
			front_face: FrontFace::CLOCKWISE,
		}
	}
}

impl Rasterization {
	pub fn set_depth_clamp_enabled(mut self, enabled: bool) -> Self {
		self.depth_clamp_enabled = enabled;
		self
	}

	pub fn set_depth_bias(mut self, bias: Option<DepthBias>) -> Self {
		self.depth_bias = bias;
		self
	}

	pub fn set_rasterizer_discard_enabled(mut self, enabled: bool) -> Self {
		self.rasterizer_discard_enabled = enabled;
		self
	}

	pub fn set_polygon_mode(mut self, mode: PolygonMode) -> Self {
		self.polygon_mode = mode;
		self
	}

	pub fn set_line_width(mut self, width: f32) -> Self {
		self.line_width = width;
		self
	}

	pub fn set_cull_mode(mut self, mode: CullMode) -> Self {
		self.cull_mode = mode;
		self
	}

	pub fn set_front_face(mut self, mode: FrontFace) -> Self {
		self.front_face = mode;
		self
	}
}

impl Into<backend::vk::PipelineRasterizationStateCreateInfo> for Rasterization {
	fn into(self) -> backend::vk::PipelineRasterizationStateCreateInfo {
		let mut info = backend::vk::PipelineRasterizationStateCreateInfo::builder()
			.depth_clamp_enable(self.depth_clamp_enabled)
			.rasterizer_discard_enable(self.rasterizer_discard_enabled)
			.polygon_mode(self.polygon_mode)
			.cull_mode(self.cull_mode)
			.front_face(self.front_face)
			.line_width(self.line_width)
			.depth_bias_enable(self.depth_bias.is_some());
		if let Some(bias) = &self.depth_bias {
			info = info
				.depth_bias_constant_factor(bias.constant_factor)
				.depth_bias_clamp(bias.clamp)
				.depth_bias_slope_factor(bias.slope_factor);
		}
		info.build()
	}
}
