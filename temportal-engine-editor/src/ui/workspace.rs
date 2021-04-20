use crate::{asset, ui};
use imgui::{self, im_str};
use std::{cell::RefCell, rc::Rc};
use temportal_engine as engine;

pub struct Workspace {}

impl Workspace {
	pub fn new() -> Rc<RefCell<Workspace>> {
		Rc::new(RefCell::new(Workspace {}))
	}
}

impl ui::Element for Workspace {
	fn render(&mut self, engine: &mut engine::Engine, ui: &imgui::Ui) {
		if let Some(bar) = ui.begin_main_menu_bar() {
			ui.menu(im_str!("General"), true, || {
				if imgui::MenuItem::new(im_str!("Build")).build(&ui) {
					match asset::build(engine, "demo-triangle") {
						Ok(_) => {}
						Err(e) => println!("{:?}", e),
					}
				}
			});
			bar.end(&ui);
		}
	}
}
