use crate::{backend, flags};

pub struct Multisampling {
	sample_count: flags::SampleCount,
	sample_shading: Option<f32>,
}

impl Default for Multisampling {
	fn default() -> Self {
		Self {
			sample_count: flags::SampleCount::_1,
			sample_shading: None,
		}
	}
}

impl Multisampling {
	pub fn with_sample_count(mut self, count: flags::SampleCount) -> Self {
		self.sample_count = count;
		self
	}

	pub fn with_sample_shading(mut self, shading: Option<f32>) -> Self {
		self.sample_shading = shading;
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::PipelineMultisampleStateCreateInfo {
		backend::vk::PipelineMultisampleStateCreateInfo::builder()
			.rasterization_samples(self.sample_count.into())
			.sample_shading_enable(self.sample_shading.is_some())
			.min_sample_shading(self.sample_shading.unwrap_or(0.0))
			.build()
	}
}
