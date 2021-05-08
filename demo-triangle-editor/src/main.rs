use demo_triangle;
use engine::utility::VoidResult;
use temportal_engine as engine;
use temportal_engine_editor as editor;

fn main() -> VoidResult {
	engine::logging::init(demo_triangle::name())?;

	let editor = editor::Editor::new(demo_triangle::name())?;
	if editor.borrow().run_commandlets()? {
		return Ok(());
	}

	let mut display = engine::display::Manager::new()?;
	let mut ui = editor::ui::Ui::new(&display, "Triangle Editor", 1280, 720)?;

	let workspace = editor::ui::Workspace::new(&editor.borrow());
	ui.add_element(&workspace);

	while !display.should_quit() {
		display.poll_all_events()?;
		ui.render_frame(&mut editor.borrow_mut(), display.event_pump()?)?;
	}

	Ok(())
}
