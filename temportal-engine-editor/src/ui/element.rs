use imgui;
use temportal_engine as engine;

pub trait Element {
	fn render(&mut self, engine: &mut engine::Engine, ui: &imgui::Ui);
}
