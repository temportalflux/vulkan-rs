use crate::{asset, ui};
use imgui::{self, im_str};
use std::{cell::RefCell, rc::Rc};

pub struct Workspace {
	simulation: ui::windows::Simulation,
}

impl Workspace {
	pub fn new(editor: &crate::Editor) -> Rc<RefCell<Workspace>> {
		Rc::new(RefCell::new(Workspace {
			simulation: ui::windows::Simulation::new(editor),
		}))
	}
}

impl ui::Element for Workspace {
	fn render(&mut self, editor: &mut crate::Editor, ui: &imgui::Ui) {
		if let Some(bar) = ui.begin_main_menu_bar() {
			ui.menu(im_str!("General"), true, || {
				let build = imgui::MenuItem::new(im_str!("Build")).build(&ui);
				let rebuild = imgui::MenuItem::new(im_str!("Build (Force)")).build(&ui);
				for app_module in editor.modules.iter() {
					if build || rebuild {
						match asset::build(editor.asset_manager(), &app_module.location, rebuild) {
							Ok(_) => {}
							Err(e) => log::error!(target: "ui", "Failed to build... {:?}", e),
						}
					}
					if imgui::MenuItem::new(im_str!("Package")).build(&ui) {
						match asset::package(&app_module.name, &app_module.location) {
							Ok(_) => {}
							Err(e) => log::error!(target: "ui", "Failed to package... {:?}", e),
						}
					}
				}
			});
			ui.menu(im_str!("Windows"), true, || {
				if imgui::MenuItem::new(im_str!("Simulation")).build(&ui) {
					self.simulation.open_or_bring_to_front(editor);
				}
			});
			bar.end(&ui);
		}
		self.simulation.render(editor, ui);
	}
}
