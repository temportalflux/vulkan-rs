use crate::{backend, flags};

pub struct Multisampling {
	enable_sample_shading: bool,
	sample_count: flags::SampleCount,
	min_sample_shading: f32,
}

impl Default for Multisampling {
	fn default() -> Self {
		Self {
			enable_sample_shading: false,
			sample_count: flags::SampleCount::_1,
			min_sample_shading: 0.0,
		}
	}
}

impl Multisampling {
	pub fn enable_sample_shading(mut self) -> Self {
		self.enable_sample_shading = true;
		self
	}

	pub fn with_sample_count(mut self, count: flags::SampleCount) -> Self {
		self.sample_count = count;
		self
	}

	pub fn with_minimum_shading(mut self, shading: f32) -> Self {
		self.min_sample_shading = shading;
		self
	}

	pub(crate) fn as_vk(&self) -> backend::vk::PipelineMultisampleStateCreateInfo {
		backend::vk::PipelineMultisampleStateCreateInfo::builder()
			.sample_shading_enable(self.enable_sample_shading)
			.rasterization_samples(self.sample_count.into())
			.min_sample_shading(self.min_sample_shading)
			.build()
	}
}
