use crate::utility::VulkanInfo;
use erupt;
use temportal_math::Vector;

#[derive(Copy, Clone, Debug)]
pub enum ClearValue {
	Color(Vector<f32, 4>),
	DepthStencil(f32, u32),
}

impl VulkanInfo<erupt::vk::ClearValue> for ClearValue {
	fn to_vk(&self) -> erupt::vk::ClearValue {
		match *self {
			ClearValue::Color(values) => erupt::vk::ClearValue {
				color: erupt::vk::ClearColorValue {
					float32: *values.data(),
				},
			},
			ClearValue::DepthStencil(depth, stencil) => erupt::vk::ClearValue {
				depth_stencil: erupt::vk::ClearDepthStencilValue { depth, stencil },
			},
		}
	}
}
