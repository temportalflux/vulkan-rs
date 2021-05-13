use temportal_engine as engine;
use temportal_engine_editor as editor;

fn main() -> engine::utility::VoidResult {
	let editor = editor::Editor::new::<ui::UIDemo>()?;
	let _ = editor.borrow().run_commandlets()?;
	Ok(())
}
