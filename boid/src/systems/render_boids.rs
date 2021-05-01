use crate::engine::{
	self,
	graphics::{self, command, flags, pipeline, shader, structs, RenderChain},
	utility::{self, AnyError},
	Engine,
};
use std::{cell::RefCell, rc::Rc};

pub struct RenderBoids {
	vert_shader: Rc<shader::Module>,
	frag_shader: Rc<shader::Module>,
	pipeline: Option<pipeline::Pipeline>,
	pipeline_layout: Option<pipeline::Layout>,
}

impl RenderBoids {
	pub fn new(
		engine: &Engine,
		render_chain: &mut RenderChain,
	) -> Result<Rc<RefCell<RenderBoids>>, AnyError> {
		let vert_shader = Rc::new(shader::Module::create(
			render_chain.logical().clone(),
			shader::Info {
				kind: flags::ShaderKind::Vertex,
				entry_point: String::from("main"),
				bytes: {
					let shader = engine
						.assets
						.loader
						.load_sync(
							&engine.assets.types,
							&engine.assets.library,
							&engine::asset::Id::new(crate::name(), "vertex"),
						)?
						.downcast::<engine::graphics::Shader>()
						.unwrap();
					shader.contents().clone()
				},
			},
		)?);

		let frag_shader = Rc::new(shader::Module::create(
			render_chain.logical().clone(),
			shader::Info {
				kind: flags::ShaderKind::Fragment,
				entry_point: String::from("main"),
				bytes: {
					let shader = engine
						.assets
						.loader
						.load_sync(
							&engine.assets.types,
							&engine.assets.library,
							&engine::asset::Id::new(crate::name(), "fragment"),
						)?
						.downcast::<engine::graphics::Shader>()
						.unwrap();
					shader.contents().clone()
				},
			},
		)?);

		let strong = Rc::new(RefCell::new(RenderBoids {
			pipeline_layout: None,
			pipeline: None,
			vert_shader,
			frag_shader,
		}));

		render_chain.add_render_chain_element(strong.clone())?;
		render_chain.add_command_recorder(strong.clone())?;

		Ok(strong)
	}
}

impl graphics::RenderChainElement for RenderBoids {
	fn initialize_with(&mut self, _render_chain: &graphics::RenderChain) -> utility::Result<()> {
		Ok(())
	}

	fn destroy_render_chain(&mut self, _: &graphics::RenderChain) -> utility::Result<()> {
		self.pipeline = None;
		self.pipeline_layout = None;
		Ok(())
	}

	fn on_render_chain_constructed(
		&mut self,
		render_chain: &graphics::RenderChain,
		resolution: structs::Extent2D,
	) -> utility::Result<()> {
		self.pipeline_layout =
			Some(pipeline::Layout::builder().build(render_chain.logical().clone())?);
		self.pipeline = Some(
			pipeline::Info::default()
				.add_shader(Rc::downgrade(&self.vert_shader))
				.add_shader(Rc::downgrade(&self.frag_shader))
				.with_vertex_layout(
					pipeline::vertex::Layout::default(), //	.with_object::<Vertex>(0, flags::VertexInputRate::VERTEX),
				)
				.set_viewport_state(
					pipeline::ViewportState::default()
						.add_viewport(graphics::utility::Viewport::default().set_size(resolution))
						.add_scissor(graphics::utility::Scissor::default().set_size(resolution)),
				)
				.set_rasterization_state(pipeline::RasterizationState::default())
				.set_color_blending(pipeline::ColorBlendState::default().add_attachment(
					pipeline::ColorBlendAttachment {
						color_flags: flags::ColorComponent::R
							| flags::ColorComponent::G | flags::ColorComponent::B
							| flags::ColorComponent::A,
						blend: None,
					},
				))
				.create_object(
					render_chain.logical().clone(),
					&self.pipeline_layout.as_ref().unwrap(),
					&render_chain.render_pass(),
				)?,
		);

		Ok(())
	}
}

impl graphics::CommandRecorder for RenderBoids {
	fn record_to_buffer(&self, buffer: &mut command::Buffer) -> utility::Result<()> {
		buffer.bind_pipeline(
			&self.pipeline.as_ref().unwrap(),
			flags::PipelineBindPoint::GRAPHICS,
		);
		//buffer.bind_vertex_buffers(0, vec![self.vertex_buffer.as_ref().unwrap()], vec![0]);
		//buffer.bind_index_buffer(self.index_buffer.as_ref().unwrap(), 0);
		//buffer.draw(self.indices.len(), 0, 1, 0, 0);
		Ok(())
	}
}
