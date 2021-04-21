use crate::{asset, ui};
use imgui::{self, im_str};
use std::{cell::RefCell, rc::Rc};

pub struct Workspace {
	simulation: ui::windows::Simulation,
}

impl Workspace {
	pub fn new() -> Rc<RefCell<Workspace>> {
		Rc::new(RefCell::new(Workspace {
			simulation: ui::windows::Simulation::new(),
		}))
	}
}

impl ui::Element for Workspace {
	fn render(&mut self, editor: &crate::Editor, ui: &imgui::Ui) {
		if let Some(bar) = ui.begin_main_menu_bar() {
			ui.menu(im_str!("General"), true, || {
				if imgui::MenuItem::new(im_str!("Build")).build(&ui) {
					match asset::build(editor.asset_manager(), "demo-triangle") {
						Ok(_) => {}
						Err(e) => println!("{:?}", e),
					}
				}
				if imgui::MenuItem::new(im_str!("Build & Package")).build(&ui) {
					match asset::build(editor.asset_manager(), "demo-triangle") {
						Ok(_) => {}
						Err(e) => println!("{:?}", e),
					}
					match asset::package("demo-triangle") {
						Ok(_) => {}
						Err(e) => println!("{:?}", e),
					}
				}
			});
			ui.menu(im_str!("Windows"), true, || {
				if imgui::MenuItem::new(im_str!("Simulation")).build(&ui) {
					self.simulation.open_or_bring_to_front();
				}
			});
			bar.end(&ui);
		}
		self.simulation.render(editor, ui);
	}
}
