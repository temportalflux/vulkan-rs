use crate::{
	asset,
	engine::{self, utility::AnyError, Application},
	graphics,
	settings::{self, Settings},
};
use std::{cell::RefCell, rc::Rc};

pub static EDITOR_LOG: &'static str = "Editor";

pub struct Editor {
	asset_manager: asset::Manager,
	pub settings: settings::Editor,
	pub module_name: String,
}

impl Editor {
	pub fn new<T: Application>() -> Result<Rc<RefCell<Editor>>, AnyError> {
		engine::logging::init_named(&(T::name().to_string() + "_editor"))?;

		log::info!(target: EDITOR_LOG, "Initializing editor");
		let mut editor = Editor {
			asset_manager: asset::Manager::new(),
			settings: settings::Editor::load()?,
			module_name: T::name().to_string(),
		};
		editor
			.asset_manager
			.register::<engine::graphics::Shader>(graphics::ShaderEditorMetadata::boxed());
		editor
			.asset_manager
			.register::<engine::graphics::font::Font>(graphics::FontEditorMetadata::boxed());
		editor
			.asset_manager
			.register::<engine::graphics::Texture>(graphics::TextureEditorMetadata::boxed());
		Ok(Rc::new(RefCell::new(editor)))
	}

	pub fn asset_manager(&self) -> &asset::Manager {
		&self.asset_manager
	}

	pub fn asset_manager_mut(&mut self) -> &mut asset::Manager {
		&mut self.asset_manager
	}

	pub fn run_commandlets(&self) -> Result<bool, AnyError> {
		let mut args = std::env::args();
		let should_build_assets = args.any(|arg| arg == "-build-assets");
		let should_package_assets = args.any(|arg| arg == "-package");
		if should_build_assets || should_package_assets {
			if should_build_assets {
				asset::build(
					self.asset_manager(),
					&self.module_name,
					args.any(|arg| arg == "-force"),
				)?;
			}
			if should_package_assets {
				asset::package(&self.module_name)?;
			}
			return Ok(true);
		}
		return Ok(false);
	}
}
