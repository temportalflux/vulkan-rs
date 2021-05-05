use crate::{
	engine::{
		self, asset,
		math::*,
		utility::{self, AnyError},
		Engine,
	},
	graphics::{
		self, buffer, command, flags, image, image_view, pipeline, sampler, shader, structs,
		Instance, RenderChain, Vertex,
	},
};
use std::sync::{Arc, RwLock, Weak};

#[derive(Debug)]
struct CameraViewProjection {
	view: Matrix<f32, 4, 4>,
	projection: Matrix<f32, 4, 4>,
}

pub struct RenderBoids {
	instance_count: usize,
	active_instance_buffer: Arc<buffer::Buffer>,
	frame_instance_buffers: Vec<Arc<buffer::Buffer>>,

	index_count: usize,
	index_buffer: buffer::Buffer,
	vertex_buffer: buffer::Buffer,

	image_descriptor_set: Weak<graphics::descriptor::Set>,
	image_descriptor_layout: Arc<graphics::descriptor::SetLayout>,
	image_sampler: Arc<sampler::Sampler>,
	image_view: Arc<image_view::View>,

	camera_buffers: Vec<Arc<buffer::Buffer>>,
	camera_descriptor_sets: Vec<Weak<graphics::descriptor::Set>>,
	camera_descriptor_layout: Arc<graphics::descriptor::SetLayout>,

	vert_shader: Arc<shader::Module>,
	frag_shader: Arc<shader::Module>,

	pipeline: Option<pipeline::Pipeline>,
	pipeline_layout: Option<pipeline::Layout>,

	render_chain: Arc<RenderChain>,
}

impl RenderBoids {
	pub fn new(
		engine: &Engine,
		render_chain: &mut Arc<RenderChain>,
	) -> Result<Arc<RwLock<RenderBoids>>, AnyError> {
		let vert_shader = Arc::new(Self::load_shader(
			&engine,
			&render_chain,
			engine::asset::Id::new(crate::name(), "vertex"),
		)?);
		let frag_shader = Arc::new(Self::load_shader(
			&engine,
			&render_chain,
			engine::asset::Id::new(crate::name(), "fragment"),
		)?);

		let image = Self::create_boid_image(&render_chain, Self::load_boid_texture(&engine)?)?;
		let image_view = Arc::new(Self::create_image_view(&render_chain, image)?);
		let image_sampler = Arc::new(
			sampler::Sampler::builder()
				.with_address_modes([flags::SamplerAddressMode::REPEAT; 3])
				.with_max_anisotropy(Some(render_chain.physical().max_sampler_anisotropy()))
				.build(&render_chain.logical())?,
		);

		let camera_descriptor_layout = Arc::new(
			graphics::descriptor::SetLayout::builder()
				.with_binding(
					0,
					flags::DescriptorKind::UNIFORM_BUFFER,
					1,
					flags::ShaderKind::Vertex,
				)
				.build(&render_chain.logical())?,
		);

		let frame_count = render_chain.frame_count();
		let camera_descriptor_sets = Arc::get_mut(render_chain)
			.unwrap()
			.persistent_descriptor_pool()
			.allocate_descriptor_sets(&vec![camera_descriptor_layout.clone(); frame_count])?;

		let camera_view_projection = CameraViewProjection {
			view: Matrix::identity(),
			projection: Matrix::identity(),
		};
		let mut camera_buffers = Vec::new();
		for _ in 0..frame_count {
			let buffer = graphics::buffer::Buffer::builder()
				.with_usage(flags::BufferUsage::UNIFORM_BUFFER)
				.with_size(std::mem::size_of::<CameraViewProjection>())
				.with_alloc(
					graphics::alloc::Info::default()
						.with_usage(flags::MemoryUsage::CpuToGpu)
						.requires(flags::MemoryProperty::HOST_VISIBLE)
						.requires(flags::MemoryProperty::HOST_COHERENT),
				)
				.with_sharing(flags::SharingMode::EXCLUSIVE)
				.build(&render_chain.allocator())?;

			Self::write_camera_view_proj(&buffer, &camera_view_projection)?;
			camera_buffers.push(Arc::new(buffer));
		}

		let image_descriptor_layout = Arc::new(
			graphics::descriptor::SetLayout::builder()
				.with_binding(
					0,
					flags::DescriptorKind::COMBINED_IMAGE_SAMPLER,
					1,
					flags::ShaderKind::Fragment,
				)
				.build(&render_chain.logical())?,
		);

		let image_descriptor_set = Arc::get_mut(render_chain)
			.unwrap()
			.persistent_descriptor_pool()
			.allocate_descriptor_sets(&vec![image_descriptor_layout.clone()])?
			.pop()
			.unwrap();

		let (vertex_buffer, index_buffer, index_count) = Self::create_boid_model(&render_chain)?;
		let active_instance_buffer = Arc::new(Self::create_instance_buffer(&render_chain, 0)?);

		let strong = Arc::new(RwLock::new(RenderBoids {
			render_chain: render_chain.clone(),
			pipeline_layout: None,
			pipeline: None,
			vert_shader,
			frag_shader,
			camera_descriptor_layout,
			camera_descriptor_sets,
			camera_buffers,
			image_view,
			image_sampler,
			image_descriptor_layout,
			image_descriptor_set,
			vertex_buffer,
			index_buffer,
			index_count,
			active_instance_buffer,
			frame_instance_buffers: Vec::new(),
			instance_count: 0,
		}));

		if let Some(chain) = Arc::get_mut(render_chain) {
			chain.add_render_chain_element(&strong)?;
			chain.add_command_recorder(&strong)?;
		}

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
	) -> Result<Arc<image::Image>, AnyError> {
		let image = Arc::new(
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
		image: Arc<image::Image>,
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
		let half_unit = 0.5;
		let vertices = vec![
			Vertex::default()
				.with_pos(vector![-half_unit, -half_unit])
				.with_tex_coord(vector![0.0, 0.0]),
			Vertex::default()
				.with_pos(vector![half_unit, -half_unit])
				.with_tex_coord(vector![1.0, 0.0]),
			Vertex::default()
				.with_pos(vector![half_unit, half_unit])
				.with_tex_coord(vector![1.0, 1.0]),
			Vertex::default()
				.with_pos(vector![-half_unit, half_unit])
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

	fn create_instance_buffer(
		render_chain: &RenderChain,
		instance_count: usize,
	) -> Result<buffer::Buffer, AnyError> {
		Ok(graphics::buffer::Buffer::builder()
			.with_usage(flags::BufferUsage::VERTEX_BUFFER)
			.with_usage(flags::BufferUsage::TRANSFER_DST)
			.with_size(std::mem::size_of::<Instance>() * instance_count)
			.with_alloc(
				graphics::alloc::Info::default()
					.with_usage(flags::MemoryUsage::GpuOnly)
					.requires(flags::MemoryProperty::DEVICE_LOCAL),
			)
			.with_sharing(flags::SharingMode::EXCLUSIVE)
			.build(&render_chain.allocator())?)
	}
}

impl graphics::RenderChainElement for RenderBoids {
	fn initialize_with(
		&mut self,
		render_chain: &mut graphics::RenderChain,
	) -> utility::Result<Vec<Arc<command::Semaphore>>> {
		use graphics::alloc::Object;
		use graphics::descriptor::*;

		SetUpdate::default()
			.with(UpdateOperation::Write(WriteOp {
				destination: UpdateOperationSet {
					set: self.image_descriptor_set.clone(),
					binding_index: 0,
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

		let mut camera_set_updates = SetUpdate::default();
		for (set_weak, buffer_rc) in self
			.camera_descriptor_sets
			.iter()
			.zip(self.camera_buffers.iter())
		{
			camera_set_updates = camera_set_updates.with(UpdateOperation::Write(WriteOp {
				destination: UpdateOperationSet {
					set: set_weak.clone(),
					binding_index: 0,
					array_element: 0,
				},
				kind: graphics::flags::DescriptorKind::UNIFORM_BUFFER,
				object: ObjectKind::Buffer(vec![BufferKind {
					buffer: buffer_rc.clone(),
					offset: 0,
					range: buffer_rc.size(),
				}]),
			}));
		}
		camera_set_updates.apply(&render_chain.logical());

		Ok(Vec::new())
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
				.with_descriptors(&self.camera_descriptor_layout)
				.with_descriptors(&self.image_descriptor_layout)
				.build(render_chain.logical().clone())?,
		);
		self.pipeline = Some(
			pipeline::Info::default()
				.add_shader(Arc::downgrade(&self.vert_shader))
				.add_shader(Arc::downgrade(&self.frag_shader))
				.with_vertex_layout(
					pipeline::vertex::Layout::default()
						.with_object::<Vertex>(0, flags::VertexInputRate::VERTEX)
						.with_object::<Instance>(1, flags::VertexInputRate::INSTANCE),
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
	fn record_to_buffer(&self, buffer: &mut command::Buffer, frame: usize) -> utility::Result<()> {
		buffer.bind_pipeline(
			&self.pipeline.as_ref().unwrap(),
			flags::PipelineBindPoint::GRAPHICS,
		);
		buffer.bind_descriptors(
			flags::PipelineBindPoint::GRAPHICS,
			self.pipeline_layout.as_ref().unwrap(),
			0,
			vec![
				&self.camera_descriptor_sets[frame].upgrade().unwrap(),
				&self.image_descriptor_set.upgrade().unwrap(),
			],
		);
		buffer.bind_vertex_buffers(0, vec![&self.vertex_buffer], vec![0]);
		buffer.bind_vertex_buffers(1, vec![&self.frame_instance_buffers[frame]], vec![0]);
		buffer.bind_index_buffer(&self.index_buffer, 0);
		buffer.draw(self.index_count, 0, self.instance_count, 0, 0);
		Ok(())
	}

	fn update_pre_submit(&mut self, frame: usize, _: &Vector<u32, 2>) -> utility::Result<()> {
		let camera_position = Vector::new([0.0, 0.0, -10.0]);
		let camera_orientation = Quaternion::identity();
		let camera_forward = camera_orientation.rotate(&engine::world::global_forward());
		let camera_up = camera_orientation.rotate(&engine::world::global_up());

		//let xy_aspect_ratio = (resolution.x() as f32) / (resolution.y() as f32);
		//let vertical_fov = 45.0;
		// According to this calculator http://themetalmuncher.github.io/fov-calc/
		// whose source code is https://github.com/themetalmuncher/fov-calc/blob/gh-pages/index.html#L24
		// the equation to get verticalFOV from horizontalFOV is: verticalFOV = 2 * atan(tan(horizontalFOV / 2) * height / width)
		// And by shifting the math to get horizontal from vertical, the equation is actually the same except the aspectRatio is flipped.
		//let horizontal_fov = 2.0 * f32::atan(f32::tan(vertical_fov / 2.0) * xy_aspect_ratio);
		//let near_plane = 0.1;
		//let far_plane = 100.0;

		let half_size = 15.0;

		let camera_view_projection = CameraViewProjection {
			view: Matrix::look_at(camera_position, camera_position + camera_forward, camera_up),
			projection: Matrix::orthographic(
				-half_size, half_size, -half_size, half_size, 0.01, 100.0,
			),
		};

		Self::write_camera_view_proj(&self.camera_buffers[frame], &camera_view_projection)?;

		self.frame_instance_buffers[frame] = self.active_instance_buffer.clone();

		Ok(())
	}
}

impl RenderBoids {
	fn write_camera_view_proj(
		buffer: &buffer::Buffer,
		camera: &CameraViewProjection,
	) -> utility::Result<()> {
		let mut mem = buffer.memory()?;
		let wrote_all = mem
			.write_item(camera)
			.map_err(|e| utility::Error::GraphicsBufferWrite(e))?;
		assert!(wrote_all);
		Ok(())
	}

	pub fn set_instances(&mut self, _instances: Vec<Instance>) -> utility::Result<()> {
		//graphics::TaskCopyImageToGpu::new(Arc::get_mut(&mut self.render_chain).unwrap())?
		//	.begin()?
		//	.stage(&instances[..])?
		//	.copy_stage_to_buffer(&self.active_instance_buffer)
		//	.end()?
		//	.wait_until_idle()?;
		Ok(())
	}
}
