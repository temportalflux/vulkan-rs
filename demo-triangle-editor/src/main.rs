extern crate imgui;

use demo_triangle;
use temportal_engine as engine;
use temportal_engine_editor as editor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let editor = editor::Editor::new(demo_triangle::create_engine()?);

	let display = engine::Engine::create_display_manager(editor.borrow().engine())?;
	let mut ui = editor::ui::Ui::new(&display.borrow(), "Triangle Editor", 1280, 720)?;

	let workspace = editor::ui::Workspace::new();
	ui.add_element(&workspace);

	while !display.borrow_mut().should_quit() {
		display.borrow_mut().poll_all_events()?;
		ui.render_frame(&editor.borrow(), display.borrow().event_pump()?)?;
		::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
	}

	Ok(())
}
