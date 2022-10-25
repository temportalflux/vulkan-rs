use crate::{
	backend,
	device::logical,
	flags,
	pipeline::{layout, state, Pipeline},
	renderpass, shader, utility,
};

use std::sync;

/// Information used to construct a [`Pipeline`](Pipeline).
pub struct Builder {
	shaders: Vec<sync::Weak<shader::Module>>,
	vertex_input: state::vertex::Layout,
	topology: state::Topology,
	viewport_state: state::Viewport,
	rasterization_state: state::Rasterization,
	multisampling: state::Multisampling,
	color_blending: state::color_blend::ColorBlend,
	depth_stencil: state::DepthStencil,
	dynamic_state: state::Dynamic,
	name: String,
}

impl Default for Builder {
	fn default() -> Self {
		Self {
			shaders: Vec::new(),
			vertex_input: Default::default(),
			topology: state::Topology::default(),
			viewport_state: Default::default(),
			rasterization_state: Default::default(),
			multisampling: Default::default(),
			color_blending: Default::default(),
			depth_stencil: Default::default(),
			dynamic_state: Default::default(),
			name: String::new(),
		}
	}
}

impl Builder {
	/// Adds a shader to the pipeline.
	/// You should only add 1 shader of each [`kind`](flags::ShaderKind).
	pub fn add_shader(mut self, shader: sync::Weak<shader::Module>) -> Self {
		self.shaders.push(shader);
		self
	}

	pub fn with_vertex_layout(mut self, layout: state::vertex::Layout) -> Self {
		self.vertex_input = layout;
		self
	}

	pub fn with_topology(mut self, topology: state::Topology) -> Self {
		self.topology = topology;
		self
	}

	pub fn set_viewport_state(mut self, state: state::Viewport) -> Self {
		self.viewport_state = state;
		self
	}

	pub fn set_rasterization_state(mut self, state: state::Rasterization) -> Self {
		self.rasterization_state = state;
		self
	}

	pub fn set_color_blending(mut self, info: state::color_blend::ColorBlend) -> Self {
		self.color_blending = info;
		self
	}

	pub fn with_depth_stencil(mut self, state: state::DepthStencil) -> Self {
		self.depth_stencil = state;
		self
	}

	pub fn with_dynamic_info(mut self, dynamic_state: state::Dynamic) -> Self {
		self.dynamic_state = dynamic_state;
		self
	}

	pub fn with_dynamic_state(mut self, state: flags::DynamicState) -> Self {
		self.dynamic_state = self.dynamic_state.with(state);
		self
	}

	pub fn with_multisampling(mut self, multisampling: state::Multisampling) -> Self {
		self.multisampling = multisampling;
		self
	}

	/// Creates the actual [`Pipeline`](Pipeline) object,
	/// with respect to a specific [`Render Pass`](crate::renderpass::Pass).
	pub fn build(
		self,
		device: sync::Arc<logical::Device>,
		layout: &layout::Layout,
		render_pass: &renderpass::Pass,
		subpass_index: usize,
	) -> Result<Pipeline, utility::Error> {
		use utility::HandledObject;

		let shader_stages = self
			.shaders
			.iter()
			.filter_map(|module| match module.upgrade() {
				Some(module_rc) => Some(
					backend::vk::PipelineShaderStageCreateInfo::builder()
						.stage(module_rc.kind().into())
						.module(**module_rc)
						.name(module_rc.entry_point())
						.build(),
				),
				None => None,
			})
			.collect::<Vec<_>>();
		let vertex_input = self.vertex_input.as_vk();
		let input_assembly = self.topology.as_vk();
		let viewport_state = self.viewport_state.as_vk();
		let rasterizer = self.rasterization_state.clone().into();
		let multisampling = self.multisampling.as_vk();
		let dynamic_state = self.dynamic_state.as_vk();
		let depth_stencil = self.depth_stencil.as_vk();

		let color_blending = backend::vk::PipelineColorBlendStateCreateInfo::builder()
			.logic_op_enable(false)
			.attachments(&self.color_blending.attachments)
			.build();

		let info = backend::vk::GraphicsPipelineCreateInfo::builder()
			.stages(&shader_stages)
			.vertex_input_state(&vertex_input)
			.input_assembly_state(&input_assembly)
			.viewport_state(&viewport_state)
			.rasterization_state(&rasterizer)
			.multisample_state(&multisampling)
			.color_blend_state(&color_blending)
			.depth_stencil_state(&depth_stencil)
			.dynamic_state(&dynamic_state)
			.layout(**layout)
			.render_pass(**render_pass)
			.subpass(subpass_index as u32)
			.build();

		let pipelines = match unsafe {
			device.create_graphics_pipelines(backend::vk::PipelineCache::null(), &[info], None)
		} {
			Ok(pipelines) => Ok(pipelines),
			Err((pipelines, vk_result)) => match vk_result {
				backend::vk::Result::SUCCESS => Ok(pipelines),
				_ => Err(utility::Error::VulkanError(vk_result)),
			},
		}?;
		let pipeline = Pipeline::from(device.clone(), pipelines[0]);
		device.set_object_name_logged(&pipeline.create_name(self.name.as_str()));
		Ok(pipeline)
	}
}

impl utility::NameableBuilder for Builder {
	fn set_name(&mut self, name: impl Into<String>) {
		self.name = name.into();
	}

	fn name(&self) -> &String {
		&self.name
	}
}
