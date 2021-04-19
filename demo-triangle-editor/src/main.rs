extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

use demo_triangle;
use std::{cell::RefCell, rc::Rc};
use temportal_engine::Engine;
use temportal_engine_editor::Editor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let engine = demo_triangle::create_engine()?;
	{
		let engine_mut = engine.borrow_mut();
		if engine_mut.is_build_instance() {
			return engine_mut.build();
		}
	}
	let display = Rc::new(RefCell::new(Engine::create_display_manager(&engine)?));

	let editor = Rc::new(RefCell::new(Editor::create(&display)));
	{
		(*editor.borrow_mut()).init()?;
		let weak_editor = Rc::downgrade(&editor);
		display.borrow_mut().add_event_listener(weak_editor);
	}

	editor
		.borrow_mut()
		.create_window("Triangle Editor", 1280, 720)?;

	while !engine.borrow().should_quit() {
		display.borrow_mut().poll_all_events()?;
		editor.borrow_mut().render_frame()?;
		::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
	}

	Ok(())
}
