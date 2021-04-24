use temportal_engine::{graphics, utility};
use temportal_graphics::{command, flags, pipeline, shader, structs};

pub struct TriangleRenderer {
	pipeline: Option<pipeline::Pipeline>,
	pipeline_layout: Option<pipeline::Layout>,
	vert_bytes: Vec<u8>,
	frag_bytes: Vec<u8>,
	vert_shader: Option<shader::Module>,
	frag_shader: Option<shader::Module>,
}

impl TriangleRenderer {
	pub fn new(vert_bytes: Vec<u8>, frag_bytes: Vec<u8>) -> TriangleRenderer {
		TriangleRenderer {
			pipeline_layout: None,
			pipeline: None,
			vert_bytes,
			frag_bytes,
			vert_shader: None,
			frag_shader: None,
		}
	}
}

impl graphics::RenderChainElement for TriangleRenderer {
	fn initialize_with(&mut self, render_chain: &graphics::RenderChain) -> utility::Result<()> {
		self.vert_shader = Some(utility::as_graphics_error(shader::Module::create(
			render_chain.logical().clone(),
			shader::Info {
				kind: flags::ShaderKind::Vertex,
				entry_point: String::from("main"),
				bytes: self.vert_bytes.clone(),
			},
		))?);
		self.frag_shader = Some(utility::as_graphics_error(shader::Module::create(
			render_chain.logical().clone(),
			shader::Info {
				kind: flags::ShaderKind::Fragment,
				entry_point: String::from("main"),
				bytes: self.frag_bytes.clone(),
			},
		))?);
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
		self.pipeline_layout = Some(utility::as_graphics_error(pipeline::Layout::create(
			render_chain.logical().clone(),
		))?);
		self.pipeline = Some(utility::as_graphics_error(
			pipeline::Info::default()
				.add_shader(&self.vert_shader.as_ref().unwrap())
				.add_shader(&self.frag_shader.as_ref().unwrap())
				.set_viewport_state(
					pipeline::ViewportState::default()
						.add_viewport(
							temportal_graphics::utility::Viewport::default().set_size(resolution),
						)
						.add_scissor(
							temportal_graphics::utility::Scissor::default().set_size(resolution),
						),
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
