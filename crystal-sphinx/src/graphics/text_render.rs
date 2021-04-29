use crate::engine::{
	self,
	graphics::{self, command, flags, pipeline, shader, structs, RenderChain},
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
		let asset = engine.assets.loader.load_sync(
			&engine.assets.types,
			&engine.assets.library,
			&asset_id,
		)?;
		let shader = engine::asset::as_asset::<engine::graphics::Shader>(&asset);
		self.bytes = shader.contents().clone();
		Ok(())
	}

	pub fn create_module(&mut self, render_chain: &graphics::RenderChain) -> utility::Result<()> {
		self.module = Some(Rc::new(utility::as_graphics_error(
			shader::Module::create(
				render_chain.logical().clone(),
				shader::Info {
					kind: self.kind,
					entry_point: String::from("main"),
					bytes: self.bytes.clone(),
				},
			),
		)?));
		Ok(())
	}
}

pub struct TextRender {
	pipeline: Option<pipeline::Pipeline>,
	pipeline_layout: Option<pipeline::Layout>,
	font_atlas_descriptor_set: Weak<graphics::descriptor::Set>,
	font_atlas_descriptor_layout: Option<Rc<graphics::descriptor::SetLayout>>,
	shaders: HashMap<flags::ShaderKind, ShaderItem>,
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
		let mut instance = TextRender {
			pipeline_layout: None,
			pipeline: None,
			font_atlas_descriptor_layout: None,
			font_atlas_descriptor_set: Weak::new(),
			shaders: HashMap::new(),
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
		use graphics::descriptor::*;
		let font_sampler_binding_number = 0;

		self.font_atlas_descriptor_layout = Some(Rc::new(utility::as_graphics_error(
			SetLayout::builder()
				.with_binding(
					font_sampler_binding_number,
					flags::DescriptorKind::COMBINED_IMAGE_SAMPLER,
					1,
					flags::ShaderKind::Fragment,
				)
				.build(&render_chain.logical()),
		)?));

		self.font_atlas_descriptor_set = utility::as_graphics_error(
			render_chain
				.persistent_descriptor_pool()
				.borrow_mut()
				.allocate_descriptor_sets(&vec![self
					.font_atlas_descriptor_layout
					.as_ref()
					.unwrap()
					.clone()]),
		)?
		.pop()
		.unwrap();

		SetUpdate::default()
			.with(UpdateOperation::Write(WriteOp {
				destination: UpdateOperationSet {
					set: self.font_atlas_descriptor_set.clone(),
					binding_index: 0,
					array_element: 0,
				},
				kind: graphics::flags::DescriptorKind::COMBINED_IMAGE_SAMPLER,
				objects: ObjectKind::Image(vec![/*ImageKind {
				sampler: Rc::new(),
				view: Rc::new(),
				layout: flags::ImageLayout::UNDEFINED,
			}*/]),
			}))
			.apply(&render_chain.logical());

		self.shader_item_mut(flags::ShaderKind::Vertex)
			.create_module(render_chain)?;
		self.shader_item_mut(flags::ShaderKind::Fragment)
			.create_module(render_chain)?;

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
				.add_shader(Rc::downgrade(self.shader_module(flags::ShaderKind::Vertex)))
				.add_shader(Rc::downgrade(
					self.shader_module(flags::ShaderKind::Fragment),
				))
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

impl graphics::CommandRecorder for TextRender {
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
