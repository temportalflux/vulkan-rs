use crate::engine::{
	self,
	graphics::{
		self, buffer, command, flags, image_view, pipeline, sampler, shader, structs, RenderChain,
	},
	math::Vector,
	utility::{self, AnyError},
	Engine,
};
use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

struct ShaderItem {
	kind: flags::ShaderKind,
	bytes: Vec<u8>,
	module: Option<Rc<shader::Module>>,
}

impl ShaderItem {
	pub fn load_bytes(
		&mut self,
		engine: &Engine,
		asset_id: &engine::asset::Id,
	) -> Result<(), AnyError> {
		let shader = engine
			.assets
			.loader
			.load_sync(&engine.assets.types, &engine.assets.library, &asset_id)?
			.downcast::<engine::graphics::Shader>()
			.unwrap();
		self.bytes = shader.contents().clone();
		Ok(())
	}

	pub fn create_module(&mut self, render_chain: &graphics::RenderChain) -> utility::Result<()> {
		self.module = Some(Rc::new(shader::Module::create(
			render_chain.logical().clone(),
			shader::Info {
				kind: self.kind,
				entry_point: String::from("main"),
				bytes: self.bytes.clone(),
			},
		)?));
		Ok(())
	}
}

struct Vertex {
	pos_and_width_edge: Vector<f32, 4>,
	tex_coord: Vector<f32, 4>,
	color: Vector<f32, 4>,
}

impl pipeline::vertex::Object for Vertex {
	fn attributes() -> Vec<pipeline::vertex::Attribute> {
		vec![
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Vertex, pos_and_width_edge),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Vertex, tex_coord),
				format: flags::Format::R32G32_SFLOAT,
			},
			pipeline::vertex::Attribute {
				offset: graphics::utility::offset_of!(Vertex, color),
				format: flags::Format::R32G32B32A32_SFLOAT,
			},
		]
	}
}

pub struct TextRender {
	index_buffer: Option<Rc<buffer::Buffer>>,
	vertex_buffer: Option<Rc<buffer::Buffer>>,
	indices: Vec<u32>,
	vertices: Vec<Vertex>,

	font_atlas_descriptor_set: Weak<graphics::descriptor::Set>,
	font_atlas_descriptor_layout: Option<Rc<graphics::descriptor::SetLayout>>,

	font_atlas_sampler: Rc<sampler::Sampler>,
	font_atlas_view: Rc<image_view::View>,
	shaders: HashMap<flags::ShaderKind, ShaderItem>,

	pipeline: Option<pipeline::Pipeline>,
	pipeline_layout: Option<pipeline::Layout>,
}

impl TextRender {
	fn vertex_shader_path() -> engine::asset::Id {
		engine::asset::Id::new(crate::name(), "shaders/text/vertex")
	}

	fn fragment_shader_path() -> engine::asset::Id {
		engine::asset::Id::new(crate::name(), "shaders/text/fragment")
	}

	pub fn new(
		engine: &Engine,
		render_chain: &mut RenderChain,
	) -> Result<Rc<RefCell<TextRender>>, AnyError> {
		optick::event!();

		let font_atlas_format = flags::Format::R8_SRGB;
		let font_atlas = {
			optick::event!("load-font-image");

			let font = engine
				.assets
				.loader
				.load_sync(
					&engine.assets.types,
					&engine.assets.library,
					&engine::asset::Id::new(crate::name(), "font/unispace"),
				)?
				.downcast::<engine::graphics::font::Font>()
				.unwrap();
			let font_sdf_image_data: Vec<u8> =
				font.binary().iter().flatten().map(|alpha| *alpha).collect();

			let image_size = font.size().subvec::<3>(None).with_z(1);
			let image = Rc::new(
				graphics::image::Image::builder()
					.with_alloc(
						graphics::alloc::Info::default()
							.with_usage(flags::MemoryUsage::GpuOnly)
							.requires(flags::MemoryProperty::DEVICE_LOCAL),
					)
					.with_format(font_atlas_format)
					.with_size(image_size)
					.with_usage(flags::ImageUsage::TRANSFER_DST)
					.with_usage(flags::ImageUsage::SAMPLED)
					.build(&render_chain.allocator())?,
			);

			graphics::TaskCopyImageToGpu::new(&render_chain)?
				.begin()?
				.format_image_for_write(&image)
				.stage(&font_sdf_image_data[..])?
				.copy_stage_to_image(&image)
				.format_image_for_read(&image)
				.end()?
				.wait_until_idle()?;

			image
		};

		let font_atlas_view = Rc::new(
			graphics::image_view::View::builder()
				.for_image(font_atlas.clone())
				.with_view_type(flags::ImageViewType::TYPE_2D)
				.with_format(font_atlas_format)
				.with_range(
					structs::subresource::Range::default().with_aspect(flags::ImageAspect::COLOR),
				)
				.build(&render_chain.logical())?,
		);

		let font_atlas_sampler = Rc::new(
			graphics::sampler::Sampler::builder()
				.with_address_modes([flags::SamplerAddressMode::REPEAT; 3])
				.with_max_anisotropy(Some(render_chain.physical().max_sampler_anisotropy()))
				.build(&render_chain.logical())?,
		);

		let mut instance = TextRender {
			pipeline_layout: None,
			pipeline: None,
			shaders: HashMap::new(),
			font_atlas_view,
			font_atlas_sampler,
			font_atlas_descriptor_layout: None,
			font_atlas_descriptor_set: Weak::new(),

			vertices: vec![
				Vertex {
					pos_and_width_edge: Vector::new([-1.0, -1.0, 0.5, 0.1]),
					tex_coord: Vector::new([0.0, 0.0, 0.0, 0.0]),
					color: Vector::filled(1.0),
				},
				Vertex {
					pos_and_width_edge: Vector::new([1.0, -1.0, 0.5, 0.1]),
					tex_coord: Vector::new([1.0, 0.0, 0.0, 0.0]),
					color: Vector::filled(1.0),
				},
				Vertex {
					pos_and_width_edge: Vector::new([1.0, 1.0, 0.5, 0.1]),
					tex_coord: Vector::new([1.0, 1.0, 0.0, 0.0]),
					color: Vector::filled(1.0),
				},
				Vertex {
					pos_and_width_edge: Vector::new([-1.0, 1.0, 0.5, 0.1]),
					tex_coord: Vector::new([0.0, 1.0, 0.0, 0.0]),
					color: Vector::filled(1.0),
				},
			],
			indices: vec![0, 1, 2, 2, 3, 0],
			vertex_buffer: None,
			index_buffer: None,
		};

		instance.shaders.insert(
			flags::ShaderKind::Vertex,
			ShaderItem {
				kind: flags::ShaderKind::Vertex,
				bytes: Vec::new(),
				module: None,
			},
		);
		instance.shaders.insert(
			flags::ShaderKind::Fragment,
			ShaderItem {
				kind: flags::ShaderKind::Fragment,
				bytes: Vec::new(),
				module: None,
			},
		);

		instance
			.shader_item_mut(flags::ShaderKind::Vertex)
			.load_bytes(&engine, &TextRender::vertex_shader_path())?;
		instance
			.shader_item_mut(flags::ShaderKind::Fragment)
			.load_bytes(&engine, &TextRender::fragment_shader_path())?;

		let strong = Rc::new(RefCell::new(instance));
		render_chain.add_render_chain_element(strong.clone())?;
		render_chain.add_command_recorder(strong.clone())?;

		Ok(strong)
	}
}

impl TextRender {
	fn shader_item_mut(&mut self, kind: flags::ShaderKind) -> &mut ShaderItem {
		self.shaders.get_mut(&kind).unwrap()
	}
	fn shader_module(&self, kind: flags::ShaderKind) -> &Rc<shader::Module> {
		&self.shaders.get(&kind).unwrap().module.as_ref().unwrap()
	}
}

impl graphics::RenderChainElement for TextRender {
	fn initialize_with(&mut self, render_chain: &graphics::RenderChain) -> utility::Result<()> {
		optick::event!();
		use graphics::descriptor::*;
		let font_sampler_binding_number = 0;

		self.font_atlas_descriptor_layout = Some(Rc::new(
			SetLayout::builder()
				.with_binding(
					font_sampler_binding_number,
					flags::DescriptorKind::COMBINED_IMAGE_SAMPLER,
					1,
					flags::ShaderKind::Fragment,
				)
				.build(&render_chain.logical())?,
		));

		self.font_atlas_descriptor_set = render_chain
			.persistent_descriptor_pool()
			.borrow_mut()
			.allocate_descriptor_sets(&vec![self
				.font_atlas_descriptor_layout
				.as_ref()
				.unwrap()
				.clone()])?
			.pop()
			.unwrap();

		SetUpdate::default()
			.with(UpdateOperation::Write(WriteOp {
				destination: UpdateOperationSet {
					set: self.font_atlas_descriptor_set.clone(),
					binding_index: font_sampler_binding_number,
					array_element: 0,
				},
				kind: graphics::flags::DescriptorKind::COMBINED_IMAGE_SAMPLER,
				object: ObjectKind::Image(vec![ImageKind {
					sampler: self.font_atlas_sampler.clone(),
					view: self.font_atlas_view.clone(),
					layout: flags::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
				}]),
			}))
			.apply(&render_chain.logical());

		self.shader_item_mut(flags::ShaderKind::Vertex)
			.create_module(render_chain)?;
		self.shader_item_mut(flags::ShaderKind::Fragment)
			.create_module(render_chain)?;

		self.vertex_buffer = Some(Rc::new(
			graphics::buffer::Buffer::builder()
				.with_usage(flags::BufferUsage::VERTEX_BUFFER)
				.with_usage(flags::BufferUsage::TRANSFER_DST)
				.with_size_of(&self.vertices[..])
				.with_alloc(
					graphics::alloc::Info::default()
						.with_usage(flags::MemoryUsage::GpuOnly)
						.requires(flags::MemoryProperty::DEVICE_LOCAL),
				)
				.with_sharing(flags::SharingMode::EXCLUSIVE)
				.build(&render_chain.allocator())?,
		));

		graphics::TaskCopyImageToGpu::new(&render_chain)?
			.begin()?
			.stage(&self.vertices[..])?
			.copy_stage_to_buffer(&self.vertex_buffer.as_ref().unwrap())
			.end()?
			.wait_until_idle()?;

		self.index_buffer = Some(Rc::new(
			graphics::buffer::Buffer::builder()
				.with_usage(flags::BufferUsage::INDEX_BUFFER)
				.with_usage(flags::BufferUsage::TRANSFER_DST)
				.with_size_of(&self.indices[..])
				.with_alloc(
					graphics::alloc::Info::default()
						.with_usage(flags::MemoryUsage::GpuOnly)
						.requires(flags::MemoryProperty::DEVICE_LOCAL),
				)
				.with_sharing(flags::SharingMode::EXCLUSIVE)
				.build(&render_chain.allocator())?,
		));

		graphics::TaskCopyImageToGpu::new(&render_chain)?
			.begin()?
			.stage(&self.indices[..])?
			.copy_stage_to_buffer(&self.index_buffer.as_ref().unwrap())
			.end()?
			.wait_until_idle()?;

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
		optick::event!();
		self.pipeline_layout = Some(
			pipeline::Layout::builder()
				.with_descriptors(self.font_atlas_descriptor_layout.as_ref().unwrap())
				.build(render_chain.logical().clone())?,
		);
		self.pipeline = Some(
			pipeline::Info::default()
				.add_shader(Rc::downgrade(self.shader_module(flags::ShaderKind::Vertex)))
				.add_shader(Rc::downgrade(
					self.shader_module(flags::ShaderKind::Fragment),
				))
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
						// finalColor.rgb = (`color.src` * newColor.rgb) `color.op` (`color.dst` * oldColor.rgb);
						// finalColor.a = (`alpha.src` * newColor.a) `alpha.op` (`alpha.dst` * oldColor.a);
						blend: Some(pipeline::Blend {
							// rgb = ((newColor.a) * newColor.rgb) + ((1 - newColor.a) * oldColor.rgb)
							color: pipeline::BlendExpr {
								src: flags::BlendFactor::SRC_ALPHA,
								op: flags::BlendOp::ADD,
								dst: flags::BlendFactor::ONE_MINUS_SRC_ALPHA,
							},
							// a = (1 * newColor.rgb) + (0 * oldColor.rgb)
							alpha: pipeline::BlendExpr {
								src: flags::BlendFactor::ONE,
								op: flags::BlendOp::ADD,
								dst: flags::BlendFactor::ZERO,
							},
						}),
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

impl graphics::CommandRecorder for TextRender {
	fn record_to_buffer(&self, buffer: &mut command::Buffer) -> utility::Result<()> {
		optick::event!();
		buffer.bind_pipeline(
			&self.pipeline.as_ref().unwrap(),
			flags::PipelineBindPoint::GRAPHICS,
		);
		buffer.bind_descriptors(
			flags::PipelineBindPoint::GRAPHICS,
			self.pipeline_layout.as_ref().unwrap(),
			0,
			vec![&self.font_atlas_descriptor_set.upgrade().unwrap()],
		);
		buffer.bind_vertex_buffers(0, vec![self.vertex_buffer.as_ref().unwrap()], vec![0]);
		buffer.bind_index_buffer(self.index_buffer.as_ref().unwrap(), 0);
		buffer.draw(self.indices.len(), 0, 1, 0, 0);
		Ok(())
	}
}
