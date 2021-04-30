use crate::engine::{
	self,
	graphics::{self, buffer, command, flags, pipeline, shader, structs, RenderChain},
	math::Vector,
	utility::{self, AnyError},
	Engine,
};
use std::{cell::RefCell, rc::Rc};

pub struct TriangleRenderer {
	index_buffer: Option<Rc<buffer::Buffer>>,
	vertex_buffer: Option<Rc<buffer::Buffer>>,
	vert_bytes: Vec<u8>,
	frag_bytes: Vec<u8>,
	vert_shader: Option<Rc<shader::Module>>,
	frag_shader: Option<Rc<shader::Module>>,
	pipeline: Option<pipeline::Pipeline>,
	pipeline_layout: Option<pipeline::Layout>,
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
			vertex_buffer: None,
			index_buffer: None,
		}));

		render_chain.add_render_chain_element(strong.clone())?;
		render_chain.add_command_recorder(strong.clone())?;

		Ok(strong)
	}
}

struct Vertex {
	pos: Vector<f32, 4>,
	color: Vector<f32, 4>,
}

impl pipeline::vertex::Object for Vertex {
	fn attributes() -> Vec<pipeline::vertex::Attribute> {
		vec![
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Vertex, pos),
				format: flags::Format::R32G32_SFLOAT,
			},
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Vertex, color),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
		]
	}
}

#[cfg(test)]
mod vertex_data {
	use super::*;

	#[test]
	fn alignment() {
		assert_eq!(graphics::utility::offset_of!(Vertex, pos), 0);
		assert_eq!(graphics::utility::offset_of!(Vertex, color), 16);
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

		let verticies = vec![
			Vertex {
				pos: Vector::new([0.0, -0.5, 0.0, 0.0]),
				color: Vector::new([1.0, 0.0, 0.0, 1.0]),
			},
			Vertex {
				pos: Vector::new([0.5, 0.5, 0.0, 0.0]),
				color: Vector::new([0.0, 1.0, 0.0, 1.0]),
			},
			Vertex {
				pos: Vector::new([-0.5, 0.5, 0.0, 0.0]),
				color: Vector::new([0.0, 0.0, 1.0, 1.0]),
			},
		];
		let vertex_data_size = std::mem::size_of::<Vertex>() * verticies.len();

		self.vertex_buffer = Some(Rc::new(utility::as_graphics_error(
			graphics::buffer::Buffer::builder()
				.with_usage(flags::BufferUsage::VERTEX_BUFFER)
				.with_usage(flags::BufferUsage::TRANSFER_DST)
				.with_size(vertex_data_size)
				.with_alloc(
					graphics::alloc::Info::default()
						.with_usage(flags::MemoryUsage::GpuOnly)
						.requires(flags::MemoryProperty::DEVICE_LOCAL),
				)
				.with_sharing(flags::SharingMode::EXCLUSIVE)
				.build(&render_chain.allocator()),
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
				.with_vertex_layout(
					pipeline::vertex::Layout::default()
						.with_object::<Vertex>(0, flags::VertexInputRate::VERTEX),
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
		buffer.bind_vertex_buffers(0, vec![self.vertex_buffer.as_ref().unwrap()], vec![0]);
		//cmd_buffer.draw(3, 0, 1, 0, 0);
		Ok(())
	}
}
