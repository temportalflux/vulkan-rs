use crate::{
	asset,
	graphics::{self, command, descriptor, flags, pipeline, ShaderSet},
	utility::{self, VoidResult},
};
use std::sync;

/// A grouping of pipeline and [`shader objects`](ShaderSet) that can be drawn with a set of buffers and descriptors.
/// This is largely an engine-level abstraction around the graphics pipeline and shaders that is meant
/// to take the mental load off of pipeline creation and management.
pub struct Drawable {
	pipeline: Option<pipeline::Pipeline>,
	pipeline_layout: Option<pipeline::Layout>,
	shaders: ShaderSet,
}

impl Default for Drawable {
	fn default() -> Self {
		Self {
			shaders: ShaderSet::default(),
			pipeline_layout: None,
			pipeline: None,
		}
	}
}

impl Drawable {
	/// Adds a shader by its asset id to the drawable.
	/// Offloads logic to [`ShaderSet::insert`].
	pub fn add_shader(&mut self, id: &asset::Id) -> VoidResult {
		self.shaders.insert(id)
	}

	/// Creates the shader modules from any pending shaders added via [`Drawable::add_shader`].
	/// Offloads logic to [`ShaderSet::create_modules`].
	pub fn create_shaders(&mut self, render_chain: &graphics::RenderChain) -> utility::Result<()> {
		self.shaders.create_modules(render_chain)
	}

	/// Destroys the pipeline objects so they can be recreated by [`Drawable::create_pipeline`].
	#[profiling::function]
	pub fn destroy_pipeline(&mut self, _: &graphics::RenderChain) -> utility::Result<()> {
		self.pipeline = None;
		self.pipeline_layout = None;
		Ok(())
	}

	/// Creates the [`Pipeline`](graphics::pipeline::Pipeline) objects with a provided descriptor layout and pipline info.
	#[profiling::function]
	pub fn create_pipeline(
		&mut self,
		render_chain: &graphics::RenderChain,
		descriptor_layout: Option<&sync::Arc<descriptor::SetLayout>>,
		pipeline_info: pipeline::Info,
	) -> utility::Result<()> {
		self.pipeline_layout = Some(
			match descriptor_layout {
				Some(layout) => pipeline::Layout::builder().with_descriptors(layout),
				None => pipeline::Layout::builder(),
			}
			.build(render_chain.logical().clone())?,
		);
		self.pipeline = Some(
			pipeline_info
				.add_shader(sync::Arc::downgrade(
					&self.shaders[flags::ShaderKind::Vertex],
				))
				.add_shader(sync::Arc::downgrade(
					&self.shaders[flags::ShaderKind::Fragment],
				))
				.create_object(
					render_chain.logical().clone(),
					&self.pipeline_layout.as_ref().unwrap(),
					&render_chain.render_pass(),
				)?,
		);
		Ok(())
	}

	/// Binds the drawable pipeline to the command buffer.
	pub fn bind_pipeline(&self, buffer: &mut command::Buffer) {
		buffer.bind_pipeline(
			&self.pipeline.as_ref().unwrap(),
			flags::PipelineBindPoint::GRAPHICS,
		);
	}

	/// Binds the provided descriptor sets to the buffer using the drawable pipeline layout.
	pub fn bind_descriptors(
		&self,
		buffer: &mut command::Buffer,
		descriptor_sets: Vec<&descriptor::Set>,
	) {
		buffer.bind_descriptors(
			flags::PipelineBindPoint::GRAPHICS,
			self.pipeline_layout.as_ref().unwrap(),
			0,
			descriptor_sets,
		);
	}
}