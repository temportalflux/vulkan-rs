use imgui;

pub trait Element {
	fn render(&mut self, editor: &mut crate::Editor, ui: &imgui::Ui);
}
