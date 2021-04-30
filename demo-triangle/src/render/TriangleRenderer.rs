use crate::engine::{
	self,
	graphics::{self, command, flags, pipeline, shader, structs, RenderChain},
	utility::{self, AnyError},
	Engine,
};
use std::{cell::RefCell, rc::Rc};

pub struct TriangleRenderer {
	pipeline: Option<pipeline::Pipeline>,
	pipeline_layout: Option<pipeline::Layout>,
	vert_bytes: Vec<u8>,
	frag_bytes: Vec<u8>,
	vert_shader: Option<Rc<shader::Module>>,
	frag_shader: Option<Rc<shader::Module>>,
}

impl TriangleRenderer {
	pub fn new(
		engine: &Engine,
		render_chain: &mut RenderChain,
	) -> Result<Rc<RefCell<TriangleRenderer>>, AnyError> {
		let vert_bytes: Vec<u8>;
		let frag_bytes: Vec<u8>;
		{
			{
				let asset = engine.assets.loader.load_sync(
					&engine.assets.types,
					&engine.assets.library,
					&engine::asset::Id::new("demo-triangle", "triangle_vert"),
				)?;
				let shader = engine::asset::as_asset::<engine::graphics::Shader>(&asset);
				vert_bytes = shader.contents().clone();
			}
			{
				let asset = engine.assets.loader.load_sync(
					&engine.assets.types,
					&engine.assets.library,
					&engine::asset::Id::new("demo-triangle", "triangle_frag"),
				)?;
				let shader = engine::asset::as_asset::<engine::graphics::Shader>(&asset);
				frag_bytes = shader.contents().clone();
			}
		}

		let strong = Rc::new(RefCell::new(TriangleRenderer {
			pipeline_layout: None,
			pipeline: None,
			vert_bytes,
			frag_bytes,
			vert_shader: None,
			frag_shader: None,
		}));

		render_chain.add_render_chain_element(strong.clone())?;
		render_chain.add_command_recorder(strong.clone())?;

		Ok(strong)
	}
}

impl graphics::RenderChainElement for TriangleRenderer {
	fn initialize_with(&mut self, render_chain: &graphics::RenderChain) -> utility::Result<()> {
		self.vert_shader = Some(Rc::new(utility::as_graphics_error(
			shader::Module::create(
				render_chain.logical().clone(),
				shader::Info {
					kind: flags::ShaderKind::Vertex,
					entry_point: String::from("main"),
					bytes: self.vert_bytes.clone(),
				},
			),
		)?));
		self.frag_shader = Some(Rc::new(utility::as_graphics_error(
			shader::Module::create(
				render_chain.logical().clone(),
				shader::Info {
					kind: flags::ShaderKind::Fragment,
					entry_point: String::from("main"),
					bytes: self.frag_bytes.clone(),
				},
			),
		)?));
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
		self.pipeline_layout = Some(utility::as_graphics_error(
			pipeline::Layout::builder().build(render_chain.logical().clone()),
		)?);
		self.pipeline = Some(utility::as_graphics_error(
			pipeline::Info::default()
				.add_shader(Rc::downgrade(self.vert_shader.as_ref().unwrap()))
				.add_shader(Rc::downgrade(self.frag_shader.as_ref().unwrap()))
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
					},
				))
				.create_object(
					render_chain.logical().clone(),
					&self.pipeline_layout.as_ref().unwrap(),
					&render_chain.render_pass(),
				),
		)?);

		Ok(())
	}
}

impl graphics::CommandRecorder for TriangleRenderer {
	fn record_to_buffer(&self, buffer: &mut command::Buffer) -> utility::Result<()> {
		buffer.bind_pipeline(
			&self.pipeline.as_ref().unwrap(),
			flags::PipelineBindPoint::GRAPHICS,
		);
		//cmd_buffer.draw(3, 0, 1, 0, 0);
		buffer.draw_vertices(3);
		Ok(())
	}
}
