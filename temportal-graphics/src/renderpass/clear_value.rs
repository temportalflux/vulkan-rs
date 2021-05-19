use crate::backend;
use temportal_math::Vector;

#[derive(Copy, Clone, Debug)]
pub enum ClearValue {
	Color(Vector<f32, 4>),
	DepthStencil(f32, u32),
}

impl Into<backend::vk::ClearValue> for ClearValue {
	fn into(self) -> backend::vk::ClearValue {
		match self {
			ClearValue::Color(values) => backend::vk::ClearValue {
				color: backend::vk::ClearColorValue {
					float32: *values.data(),
				},
			},
			ClearValue::DepthStencil(depth, stencil) => backend::vk::ClearValue {
				depth_stencil: backend::vk::ClearDepthStencilValue { depth, stencil },
			},
		}
	}
}
