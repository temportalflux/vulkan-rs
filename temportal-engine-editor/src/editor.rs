use crate::{asset, engine};
use std::{cell::RefCell, rc::Rc};

pub struct Editor {
	asset_manager: asset::Manager,
	engine: Rc<RefCell<engine::Engine>>,
}

impl Editor {
	pub fn new(engine: Rc<RefCell<engine::Engine>>) -> Rc<RefCell<Editor>> {
		let mut editor = Editor {
			engine,
			asset_manager: asset::Manager::new(),
		};
		editor
			.asset_manager
			.register::<engine::graphics::Shader>(asset::ShaderEditorMetadata::boxed());
		Rc::new(RefCell::new(editor))
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
