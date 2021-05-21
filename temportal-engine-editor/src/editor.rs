use crate::{
	asset,
	engine::{self, utility::AnyError, Application, EngineApp},
	graphics,
	settings::{self, Settings},
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

pub static EDITOR_LOG: &'static str = "Editor";

/// Editor-level wrapper for [`Application`] objects which are present in this runtime.
pub struct ApplicationModule {
	pub name: String,
	pub location: PathBuf,
}

impl ApplicationModule {
	pub fn new<T: Application>() -> Self {
		Self {
			name: T::name().to_string(),
			location: PathBuf::from(T::location()),
		}
	}
}

pub struct Editor {
	asset_manager: asset::Manager,
	pub settings: settings::Editor,
	pub modules: Vec<ApplicationModule>,
}

impl Editor {
	pub fn new<T: Application>() -> Result<Rc<RefCell<Editor>>, AnyError> {
		engine::logging::init_named(&(T::name().to_string() + "_editor"))?;

		log::info!(target: EDITOR_LOG, "Initializing editor");
		let mut editor = Editor {
			asset_manager: asset::Manager::new(),
			settings: settings::Editor::load()?,
			modules: vec![
				ApplicationModule::new::<EngineApp>(),
				ApplicationModule::new::<T>(),
			],
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
			for app_module in self.modules.iter() {
				if should_build_assets {
					asset::build(
						self.asset_manager(),
						&app_module.location,
						args.any(|arg| arg == "-force"),
					)?;
				}
				if should_package_assets {
					asset::package(&app_module.name, &app_module.location)?;
				}
			}
			return Ok(true);
		}
		return Ok(false);
	}
}
