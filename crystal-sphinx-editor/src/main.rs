use crystal_sphinx;
use engine::utility::VoidResult;
use temportal_engine as engine;
use temportal_engine_editor as editor;

fn name() -> &'static str {
	std::env!("CARGO_PKG_NAME")
}

fn main() -> VoidResult {
	#[cfg(feature = "profile")]
	{
		optick::start_capture();
	}
	engine::logging::init(name())?;

	let editor = editor::Editor::new(crystal_sphinx::create_engine()?, crystal_sphinx::name())?;

	let display = engine::Engine::create_display_manager(editor.borrow().engine())?;
	let mut ui = editor::ui::Ui::new(&display.borrow(), "Crystal Sphinx Editor", 1280, 720)?;

	let workspace = editor::ui::Workspace::new(&editor.borrow());
	ui.add_element(&workspace);

	while !display.borrow_mut().should_quit() {
		display.borrow_mut().poll_all_events()?;
		ui.render_frame(&mut editor.borrow_mut(), display.borrow().event_pump()?)?;
	}

	#[cfg(feature = "profile")]
	{
		optick::stop_capture(name());
	}
	Ok(())
}
