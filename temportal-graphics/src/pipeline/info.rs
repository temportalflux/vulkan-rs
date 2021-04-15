use crate::into_builders;
use crate::{
	device::logical,
	pipeline, shader,
	utility::{self, VulkanInfo},
};
use erupt;

pub struct Info {
	shaders: Vec<erupt::vk::PipelineShaderStageCreateInfo>,
	vertex_input: erupt::vk::PipelineVertexInputStateCreateInfo,
	input_assembly: erupt::vk::PipelineInputAssemblyStateCreateInfo,
	viewport_state: pipeline::ViewportState,
	rasterization_state: pipeline::RasterizationState,
	multisampling: erupt::vk::PipelineMultisampleStateCreateInfo,
	color_blending: erupt::vk::PipelineColorBlendStateCreateInfo,
}

impl Info {
	pub fn new() -> Info {
		Info {
			shaders: Vec::new(),
			vertex_input: erupt::vk::PipelineVertexInputStateCreateInfo::default(),
			input_assembly: erupt::vk::PipelineInputAssemblyStateCreateInfoBuilder::new()
				.topology(erupt::vk::PrimitiveTopology::TRIANGLE_LIST)
				.primitive_restart_enable(false)
				.build(),
			viewport_state: pipeline::ViewportState::new(),
			rasterization_state: pipeline::RasterizationState::new(),
			multisampling: erupt::vk::PipelineMultisampleStateCreateInfoBuilder::new()
				.sample_shading_enable(false)
				.rasterization_samples(erupt::vk::SampleCountFlagBits::_1)
				.build(),
			color_blending: erupt::vk::PipelineColorBlendStateCreateInfo::default(),
		}
	}

	pub fn add_shader(mut self, shader: &shader::Module) -> Self {
		self.shaders.push(shader.to_vk());
		self
	}

	pub fn set_viewport_state(mut self, state: pipeline::ViewportState) -> Self {
		self.viewport_state = state;
		self
	}

	pub fn set_rasterization_state(mut self, state: pipeline::RasterizationState) -> Self {
		self.rasterization_state = state;
		self
	}

	pub fn set_color_blending(mut self, info: pipeline::ColorBlendState) -> Self {
		self.color_blending = info.to_vk();
		self
	}
}

impl VulkanInfo<erupt::vk::GraphicsPipelineCreateInfo> for Info {
	fn to_vk(&self) -> erupt::vk::GraphicsPipelineCreateInfo {
		erupt::vk::GraphicsPipelineCreateInfoBuilder::new()
			.stages(into_builders!(self.shaders))
			.vertex_input_state(&self.vertex_input.into_builder())
			.input_assembly_state(&self.input_assembly.into_builder())
			.viewport_state(&self.viewport_state.to_vk().into_builder())
			.rasterization_state(&self.rasterization_state.to_vk().into_builder())
			.multisample_state(&self.multisampling.into_builder())
			.color_blend_state(&self.color_blending.into_builder())
			//.layout(pipeline_layout)
			//.render_pass(render_pass)
			.subpass(0)
			.build()
	}
}

impl Info {
	pub fn create_object(
		self,
		device: &logical::Device,
	) -> Result<pipeline::Pipeline, utility::Error> {
		let info = self.to_vk().into_builder();
		let pipelines = device.create_graphics_pipelines(vec![info])?;
		Ok(pipeline::Pipeline::from(pipelines[0]))
	}
}
