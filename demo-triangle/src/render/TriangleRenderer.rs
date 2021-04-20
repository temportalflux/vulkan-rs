use temportal_engine::{display, graphics, utility};
use temportal_graphics::{command, flags, pipeline, shader};

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
	fn initialize_with(&mut self, window: &display::Window) -> utility::Result<()> {
		self.vert_shader = Some(utility::as_graphics_error(shader::Module::create(
			window.logical().clone(),
			shader::Info {
				kind: flags::ShaderKind::Vertex,
				entry_point: String::from("main"),
				bytes: self.vert_bytes.clone(),
			},
		))?);
		self.frag_shader = Some(utility::as_graphics_error(shader::Module::create(
			window.logical().clone(),
			shader::Info {
				kind: flags::ShaderKind::Fragment,
				entry_point: String::from("main"),
				bytes: self.frag_bytes.clone(),
			},
		))?);
		Ok(())
	}
	fn on_render_chain_constructed(&mut self, window: &display::Window) -> utility::Result<()> {
		println!("Render chain constructed");

		self.pipeline_layout = Some(utility::as_graphics_error(pipeline::Layout::create(
			window.logical().clone(),
		))?);
		self.pipeline = Some(utility::as_graphics_error(
			pipeline::Info::default()
				.add_shader(&self.vert_shader.as_ref().unwrap())
				.add_shader(&self.frag_shader.as_ref().unwrap())
				.set_viewport_state(
					pipeline::ViewportState::default()
						.add_viewport(
							temportal_graphics::utility::Viewport::default()
								.set_size(window.physical().image_extent()),
						)
						.add_scissor(
							temportal_graphics::utility::Scissor::default()
								.set_size(window.physical().image_extent()),
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
					window.logical().clone(),
					&self.pipeline_layout.as_ref().unwrap(),
					&window.render_pass(),
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
