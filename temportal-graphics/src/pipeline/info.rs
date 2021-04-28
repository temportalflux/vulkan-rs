use crate::{
	backend,
	device::logical,
	pipeline, renderpass, shader,
	utility::{self, VulkanInfo, VulkanObject},
};

use std::rc::{Rc, Weak};

/// Information used to construct a [`Pipeline`](pipeline::Pipeline).
pub struct Info {
	shaders: Vec<Weak<shader::Module>>,
	vertex_input: backend::vk::PipelineVertexInputStateCreateInfo,
	input_assembly: backend::vk::PipelineInputAssemblyStateCreateInfo,
	viewport_state: pipeline::ViewportState,
	rasterization_state: pipeline::RasterizationState,
	multisampling: backend::vk::PipelineMultisampleStateCreateInfo,
	color_blending: pipeline::ColorBlendState,
}

impl Default for Info {
	fn default() -> Info {
		Info {
			shaders: Vec::new(),
			vertex_input: backend::vk::PipelineVertexInputStateCreateInfo::default(),
			input_assembly: backend::vk::PipelineInputAssemblyStateCreateInfo::builder()
				.topology(backend::vk::PrimitiveTopology::TRIANGLE_LIST)
				.primitive_restart_enable(false)
				.build(),
			viewport_state: pipeline::ViewportState::default(),
			rasterization_state: pipeline::RasterizationState::default(),
			multisampling: backend::vk::PipelineMultisampleStateCreateInfo::builder()
				.sample_shading_enable(false)
				.rasterization_samples(crate::flags::SampleCount::TYPE_1)
				.build(),
			color_blending: pipeline::ColorBlendState::default(),
		}
	}
}

impl Info {
	pub fn add_shader(mut self, shader: Weak<shader::Module>) -> Self {
		self.shaders.push(shader);
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
	/// Creates the actual [`Pipeline`](pipeline::Pipeline) object,
	/// with respect to a specific [`Render Pass`](crate::renderpass::Pass).
	pub fn create_object(
		self,
		device: Rc<logical::Device>,
		layout: &pipeline::Layout,
		render_pass: &renderpass::Pass,
	) -> Result<pipeline::Pipeline, utility::Error> {
		let shader_stages = self
			.shaders
			.iter()
			.filter_map(|module| match module.upgrade() {
				Some(module_rc) => Some(module_rc.to_vk()),
				None => None,
			})
			.collect::<Vec<_>>();
		let viewport_state = self.viewport_state.to_vk();
		let rasterizer = self.rasterization_state.to_vk();

		let color_blending = backend::vk::PipelineColorBlendStateCreateInfo::builder()
			.logic_op_enable(false)
			.attachments(&self.color_blending.attachments)
			.build();

		let info = backend::vk::GraphicsPipelineCreateInfo::builder()
			.stages(&shader_stages)
			.vertex_input_state(&self.vertex_input)
			.input_assembly_state(&self.input_assembly)
			.viewport_state(&viewport_state)
			.rasterization_state(&rasterizer)
			.multisample_state(&self.multisampling)
			.color_blend_state(&color_blending)
			.layout(*layout.unwrap())
			.render_pass(*render_pass.unwrap())
			.subpass(0)
			.build();

		let pipelines =
			device.create_graphics_pipelines(backend::vk::PipelineCache::null(), &[info])?;
		Ok(pipeline::Pipeline::from(device, pipelines[0]))
	}
}
