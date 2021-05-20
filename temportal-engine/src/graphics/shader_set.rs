use crate::{
	asset,
	graphics::{self, flags, shader},
	utility::{self, VoidResult},
};
use std::{collections::HashMap, sync};

/// A discrete collection of shaders which will be or have been created on the GPU.
/// This is an engine-level abstraction to encapsulate the creation and management
/// of [`shader modules`](shader::Module).
/// Only 1 shader per [`kind`](flags::ShaderKind) may be present at the same time.
pub struct ShaderSet {
	shaders: HashMap<flags::ShaderKind, sync::Arc<shader::Module>>,
	pending_shaders: HashMap<flags::ShaderKind, Vec<u8>>,
}

impl Default for ShaderSet {
	fn default() -> Self {
		Self {
			pending_shaders: HashMap::new(),
			shaders: HashMap::new(),
		}
	}
}

impl ShaderSet {
	/// Adds a [`shader asset`](graphics::Shader) to the set.
	/// If a shader of the same kind already exists, it will be dropped the next time
	/// [`create_modules`](ShaderSet::create_modules) is called.
	#[profiling::function]
	pub fn insert(&mut self, id: &asset::Id) -> VoidResult {
		let shader = asset::Loader::load_sync(&id)?
			.downcast::<graphics::Shader>()
			.unwrap();
		self.pending_shaders
			.insert(shader.kind(), shader.contents().clone());
		Ok(())
	}

	/// Creates [`shader modules`](shader::Module) from pending shaders added by [`insert`](ShaderSet::insert),
	/// thereby dropping any existing modules with the same kind.
	#[profiling::function]
	pub fn create_modules(&mut self, render_chain: &graphics::RenderChain) -> utility::Result<()> {
		for (kind, binary) in self.pending_shaders.drain() {
			self.shaders.insert(
				kind,
				sync::Arc::new(shader::Module::create(
					render_chain.logical().clone(),
					shader::Info {
						kind: kind,
						entry_point: String::from("main"),
						bytes: binary,
					},
				)?),
			);
		}
		Ok(())
	}
}

impl std::ops::Index<flags::ShaderKind> for ShaderSet {
	type Output = sync::Arc<shader::Module>;
	fn index(&self, kind: flags::ShaderKind) -> &Self::Output {
		self.shaders.get(&kind).unwrap()
	}
}
