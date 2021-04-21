use imgui;

pub trait Element {
	fn render(&mut self, editor: &crate::Editor, ui: &imgui::Ui);
}
