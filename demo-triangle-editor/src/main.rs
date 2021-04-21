extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

use demo_triangle;
use temportal_engine_editor as editor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let engine = demo_triangle::create_engine()?;
	let editor = editor::Editor::new(&engine);

	{
		let mut editor_mut = editor.borrow_mut();
		editor_mut.init_display()?;
		editor_mut.create_window("Triangle Editor", 1280, 720)?;
	}

	let workspace = editor::ui::Workspace::new();
	editor.borrow_mut().add_element(&workspace);

	{
		let mut editor_mut = editor.borrow_mut();
		while !editor_mut.display().borrow_mut().should_quit() {
			editor_mut.display().borrow_mut().poll_all_events()?;
			editor_mut.render_frame(&mut engine.borrow_mut())?;
			::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
		}
	}

	Ok(())
}