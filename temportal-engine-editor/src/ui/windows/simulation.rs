use crate::{settings::Settings, ui, Editor};
use imgui::{self, im_str};

pub struct Simulation {
	id: String,
	is_open: bool,
	bring_to_front: bool,
}

impl Simulation {
	pub fn new(editor: &Editor) -> Simulation {
		let mut value = Simulation {
			id: "simulation".to_string(),
			is_open: false,
			bring_to_front: true,
		};
		value.is_open = editor.settings.is_window_open(&value.id);
		value
	}

	pub fn open_or_bring_to_front(&mut self, editor: &mut Editor) {
		self.is_open = true;
		self.bring_to_front();
		self.save_open_state(editor);
	}

	pub fn bring_to_front(&mut self) {
		self.bring_to_front = true;
	}

	fn save_open_state(&self, editor: &mut Editor) {
		editor.settings.set_window_open(&self.id, self.is_open);
		if let Err(e) = editor.settings.save() {
			println!("Failed to save editor settings, {}", e);
		}
	}
}

impl ui::Element for Simulation {
	fn render(&mut self, editor: &mut Editor, ui: &imgui::Ui) {
		if !self.is_open {
			return;
		}
		let was_open = self.is_open;
		imgui::Window::new(im_str!("Simulation"))
			.size([960.0, 540.0], imgui::Condition::FirstUseEver)
			.opened(&mut self.is_open)
			.focused(self.bring_to_front)
			.build(&ui, || {});
		self.bring_to_front = false;
		if self.is_open != was_open {
			self.save_open_state(editor);
		}
	}
}
