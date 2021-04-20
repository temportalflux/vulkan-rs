use imgui;
use temportal_engine as engine;

pub trait Element {
	fn render(
		&mut self,
		engine: &mut engine::Engine,
		asset_manager: &crate::asset::Manager,
		ui: &imgui::Ui,
	);
}
