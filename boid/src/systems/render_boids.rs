use crate::{
	engine::{
		self, asset,
		math::vector,
		utility::{self, AnyError},
		Engine,
	},
	graphics::{
		self, buffer, command, flags, image, image_view, pipeline, sampler, shader, structs,
		Instance, RenderChain, Vertex,
	},
};
use std::{
	cell::RefCell,
	rc::{Rc, Weak},
};

pub struct RenderBoids {
	index_count: usize,
	index_buffer: buffer::Buffer,
	vertex_buffer: buffer::Buffer,

	image_descriptor_set: Weak<graphics::descriptor::Set>,
	image_descriptor_layout: Rc<graphics::descriptor::SetLayout>,
	image_descriptor_binding: u32,
	image_sampler: Rc<sampler::Sampler>,
	image_view: Rc<image_view::View>,

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
		let vert_shader = Rc::new(Self::load_shader(
			&engine,
			&render_chain,
			engine::asset::Id::new(crate::name(), "vertex"),
		)?);
		let frag_shader = Rc::new(Self::load_shader(
			&engine,
			&render_chain,
			engine::asset::Id::new(crate::name(), "fragment"),
		)?);

		let image = Self::create_boid_image(&render_chain, Self::load_boid_texture(&engine)?)?;
		let image_view = Rc::new(Self::create_image_view(&render_chain, image)?);
		let image_sampler = Rc::new(
			sampler::Sampler::builder()
				.with_address_modes([flags::SamplerAddressMode::REPEAT; 3])
				.with_max_anisotropy(Some(render_chain.physical().max_sampler_anisotropy()))
				.build(&render_chain.logical())?,
		);

		let image_descriptor_binding = 0;

		let image_descriptor_layout = Rc::new(
			graphics::descriptor::SetLayout::builder()
				.with_binding(
					image_descriptor_binding,
					flags::DescriptorKind::COMBINED_IMAGE_SAMPLER,
					1,
					flags::ShaderKind::Fragment,
				)
				.build(&render_chain.logical())?,
		);

		let image_descriptor_set = render_chain
			.persistent_descriptor_pool()
			.borrow_mut()
			.allocate_descriptor_sets(&vec![image_descriptor_layout.clone()])?
			.pop()
			.unwrap();

		let (vertex_buffer, index_buffer, index_count) = Self::create_boid_model(&render_chain)?;

		let strong = Rc::new(RefCell::new(RenderBoids {
			pipeline_layout: None,
			pipeline: None,
			vert_shader,
			frag_shader,
			image_view,
			image_sampler,
			image_descriptor_binding,
			image_descriptor_layout,
			image_descriptor_set,
			vertex_buffer,
			index_buffer,
			index_count,
		}));

		render_chain.add_render_chain_element(strong.clone())?;
		render_chain.add_command_recorder(strong.clone())?;

		Ok(strong)
	}

	fn load_shader(
		engine: &Engine,
		render_chain: &RenderChain,
		id: asset::Id,
	) -> Result<shader::Module, AnyError> {
		let shader = engine
			.assets
			.loader
			.load_sync(&engine.assets.types, &engine.assets.library, &id)?
			.downcast::<engine::graphics::Shader>()
			.unwrap();

		Ok(shader::Module::create(
			render_chain.logical().clone(),
			shader::Info {
				kind: shader.kind(),
				entry_point: String::from("main"),
				bytes: shader.contents().clone(),
			},
		)?)
	}

	fn load_boid_texture(engine: &Engine) -> Result<Box<graphics::Texture>, AnyError> {
		Ok(engine
			.assets
			.loader
			.load_sync(
				&engine.assets.types,
				&engine.assets.library,
				&engine::asset::Id::new(crate::name(), "boid"),
			)?
			.downcast::<graphics::Texture>()
			.unwrap())
	}

	fn create_boid_image(
		render_chain: &RenderChain,
		texture: Box<graphics::Texture>,
	) -> Result<Rc<image::Image>, AnyError> {
		let image = Rc::new(
			graphics::image::Image::builder()
				.with_alloc(
					graphics::alloc::Info::default()
						.with_usage(flags::MemoryUsage::GpuOnly)
						.requires(flags::MemoryProperty::DEVICE_LOCAL),
				)
				.with_format(flags::Format::R8G8B8A8_SRGB)
				.with_size(texture.size().subvec::<3>(None).with_z(1))
				.with_usage(flags::ImageUsage::TRANSFER_DST)
				.with_usage(flags::ImageUsage::SAMPLED)
				.build(&render_chain.allocator())?,
		);
		graphics::TaskCopyImageToGpu::new(&render_chain)?
			.begin()?
			.format_image_for_write(&image)
			.stage(&texture.binary()[..])?
			.copy_stage_to_image(&image)
			.format_image_for_read(&image)
			.end()?
			.wait_until_idle()?;
		Ok(image)
	}

	fn create_image_view(
		render_chain: &RenderChain,
		image: Rc<image::Image>,
	) -> Result<image_view::View, AnyError> {
		Ok(image_view::View::builder()
			.with_format(image.format())
			.for_image(image.clone())
			.with_view_type(flags::ImageViewType::TYPE_2D)
			.with_range(
				structs::subresource::Range::default().with_aspect(flags::ImageAspect::COLOR),
			)
			.build(&render_chain.logical())?)
	}

	fn create_boid_model(
		render_chain: &RenderChain,
	) -> Result<(buffer::Buffer, buffer::Buffer, usize), AnyError> {
		let vertices = vec![
			Vertex::default()
				.with_pos(vector![-0.5, -0.5])
				.with_tex_coord(vector![0.0, 0.0]),
			Vertex::default()
				.with_pos(vector![0.5, -0.5])
				.with_tex_coord(vector![1.0, 0.0]),
			Vertex::default()
				.with_pos(vector![0.5, 0.5])
				.with_tex_coord(vector![1.0, 1.0]),
			Vertex::default()
				.with_pos(vector![-0.5, 0.5])
				.with_tex_coord(vector![0.0, 1.0]),
		];
		let indices = vec![0, 1, 2, 2, 3, 0];

		let vertex_buffer = graphics::buffer::Buffer::builder()
			.with_usage(flags::BufferUsage::VERTEX_BUFFER)
			.with_usage(flags::BufferUsage::TRANSFER_DST)
			.with_size_of(&vertices[..])
			.with_alloc(
				graphics::alloc::Info::default()
					.with_usage(flags::MemoryUsage::GpuOnly)
					.requires(flags::MemoryProperty::DEVICE_LOCAL),
			)
			.with_sharing(flags::SharingMode::EXCLUSIVE)
			.build(&render_chain.allocator())?;

		graphics::TaskCopyImageToGpu::new(&render_chain)?
			.begin()?
			.stage(&vertices[..])?
			.copy_stage_to_buffer(&vertex_buffer)
			.end()?
			.wait_until_idle()?;

		let index_buffer = graphics::buffer::Buffer::builder()
			.with_usage(flags::BufferUsage::INDEX_BUFFER)
			.with_usage(flags::BufferUsage::TRANSFER_DST)
			.with_size_of(&indices[..])
			.with_alloc(
				graphics::alloc::Info::default()
					.with_usage(flags::MemoryUsage::GpuOnly)
					.requires(flags::MemoryProperty::DEVICE_LOCAL),
			)
			.with_sharing(flags::SharingMode::EXCLUSIVE)
			.build(&render_chain.allocator())?;

		graphics::TaskCopyImageToGpu::new(&render_chain)?
			.begin()?
			.stage(&indices[..])?
			.copy_stage_to_buffer(&index_buffer)
			.end()?
			.wait_until_idle()?;

		Ok((vertex_buffer, index_buffer, indices.len()))
	}
}

impl graphics::RenderChainElement for RenderBoids {
	fn initialize_with(&mut self, render_chain: &graphics::RenderChain) -> utility::Result<()> {
		use graphics::descriptor::*;

		SetUpdate::default()
			.with(UpdateOperation::Write(WriteOp {
				destination: UpdateOperationSet {
					set: self.image_descriptor_set.clone(),
					binding_index: self.image_descriptor_binding,
					array_element: 0,
				},
				kind: graphics::flags::DescriptorKind::COMBINED_IMAGE_SAMPLER,
				object: ObjectKind::Image(vec![ImageKind {
					view: self.image_view.clone(),
					sampler: self.image_sampler.clone(),
					layout: flags::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
				}]),
			}))
			.apply(&render_chain.logical());

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
		self.pipeline_layout = Some(
			pipeline::Layout::builder()
				.with_descriptors(&self.image_descriptor_layout)
				.build(render_chain.logical().clone())?,
		);
		self.pipeline = Some(
			pipeline::Info::default()
				.add_shader(Rc::downgrade(&self.vert_shader))
				.add_shader(Rc::downgrade(&self.frag_shader))
				.with_vertex_layout(
					pipeline::vertex::Layout::default()
						.with_object::<Vertex>(0, flags::VertexInputRate::VERTEX)
						//.with_object::<Instance>(1, flags::VertexInputRate::INSTANCE),
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

impl graphics::CommandRecorder for RenderBoids {
	fn record_to_buffer(&self, buffer: &mut command::Buffer) -> utility::Result<()> {
		buffer.bind_pipeline(
			&self.pipeline.as_ref().unwrap(),
			flags::PipelineBindPoint::GRAPHICS,
		);
		buffer.bind_descriptors(
			flags::PipelineBindPoint::GRAPHICS,
			self.pipeline_layout.as_ref().unwrap(),
			0,
			vec![&self.image_descriptor_set.upgrade().unwrap()],
		);
		buffer.bind_vertex_buffers(0, vec![&self.vertex_buffer], vec![0]);
		buffer.bind_index_buffer(&self.index_buffer, 0);
		buffer.draw(self.index_count, 0, 1, 0, 0);
		Ok(())
	}
}
