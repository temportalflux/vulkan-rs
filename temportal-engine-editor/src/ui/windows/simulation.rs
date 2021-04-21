use crate::ui;
use imgui::{self, im_str};

pub struct Simulation {
	is_open: bool,
	bring_to_front: bool,
}

impl Simulation {
	pub fn new() -> Simulation {
		Simulation {
			is_open: false,
			bring_to_front: true,
		}
	}

	pub fn open_or_bring_to_front(&mut self) {
		self.is_open = true;
		self.bring_to_front();
	}

	pub fn bring_to_front(&mut self) {
		self.bring_to_front = true;
	}
}

impl ui::Element for Simulation {
	fn render(&mut self, _editor: &crate::Editor, ui: &imgui::Ui) {
		if !self.is_open {
			return;
		}
		imgui::Window::new(im_str!("Simulation"))
			.size([960.0, 540.0], imgui::Condition::FirstUseEver)
			.opened(&mut self.is_open)
			.focused(self.bring_to_front)
			.build(&ui, || {});
		self.bring_to_front = false;
	}
}
