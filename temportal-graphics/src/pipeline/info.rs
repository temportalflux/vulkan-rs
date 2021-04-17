use crate::into_builders;
use crate::{
	device::logical,
	pipeline, renderpass, shader,
	utility::{self, VulkanInfo, VulkanObject},
};
use erupt;

pub struct Info {
	shaders: Vec<erupt::vk::PipelineShaderStageCreateInfo>,
	vertex_input: erupt::vk::PipelineVertexInputStateCreateInfo,
	input_assembly: erupt::vk::PipelineInputAssemblyStateCreateInfo,
	viewport_state: pipeline::ViewportState,
	rasterization_state: pipeline::RasterizationState,
	multisampling: erupt::vk::PipelineMultisampleStateCreateInfo,
	color_blending: pipeline::ColorBlendState,
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
			color_blending: pipeline::ColorBlendState::new(),
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
		self.color_blending = info;
		self
	}
}

impl Info {
	pub fn create_object(
		self,
		device: &logical::Device,
		render_pass: &renderpass::Pass,
	) -> Result<pipeline::Pipeline, utility::Error> {
		let shader_stages = into_builders!(self.shaders);
		let vertex_input = self.vertex_input.into_builder();
		let input_assembly = self.input_assembly.into_builder();
		let viewport_state = self.viewport_state.to_vk().into_builder();
		let rasterizer = self.rasterization_state.to_vk().into_builder();
		let multisampling = self.multisampling.into_builder();

		let color_blend_attachments = into_builders!(self.color_blending.attachments);
		let color_blending = erupt::vk::PipelineColorBlendStateCreateInfoBuilder::new()
			.logic_op_enable(false)
			.attachments(color_blend_attachments)
			.build();

		let pipeline_layout = device
			.create_pipeline_layout(erupt::vk::PipelineLayoutCreateInfoBuilder::new().build())?;
		let info = erupt::vk::GraphicsPipelineCreateInfoBuilder::new()
			.stages(&shader_stages)
			.vertex_input_state(&vertex_input)
			.input_assembly_state(&input_assembly)
			.viewport_state(&viewport_state)
			.rasterization_state(&rasterizer)
			.multisample_state(&multisampling)
			.color_blend_state(&color_blending)
			.layout(pipeline_layout)
			.render_pass(*render_pass.unwrap())
			.subpass(0);

		let pipelines = device.create_graphics_pipelines(&[info])?;
		Ok(pipeline::Pipeline::from(pipelines[0]))
	}
}
