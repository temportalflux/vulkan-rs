use crate::{
	asset, engine,
	settings::{self, Settings},
};
use std::{cell::RefCell, rc::Rc};

pub static EDITOR_LOG: &'static str = "Editor";

pub struct Editor {
	asset_manager: asset::Manager,
	engine: Rc<RefCell<engine::Engine>>,
	pub settings: settings::Editor,
}

impl Editor {
	pub fn new(
		engine: Rc<RefCell<engine::Engine>>,
	) -> Result<Rc<RefCell<Editor>>, engine::utility::AnyError> {
		log::info!(target: EDITOR_LOG, "Initializing editor");
		let mut editor = Editor {
			engine,
			asset_manager: asset::Manager::new(),
			settings: settings::Editor::load()?,
		};
		editor
			.asset_manager
			.register::<engine::graphics::Shader>(asset::ShaderEditorMetadata::boxed());
		Ok(Rc::new(RefCell::new(editor)))
	}

	pub fn engine(&self) -> &Rc<RefCell<engine::Engine>> {
		&self.engine
	}

	pub fn asset_manager(&self) -> &asset::Manager {
		&self.asset_manager
	}

	pub fn asset_manager_mut(&mut self) -> &mut asset::Manager {
		&mut self.asset_manager
	}
}
