use boid;
use engine::utility::VoidResult;
use temportal_engine as engine;
use temportal_engine_editor as editor;

fn main() -> VoidResult {
	let editor = editor::Editor::new::<boid::BoidDemo>()?;
	if editor.borrow().run_commandlets()? {
		return Ok(());
	}

	Ok(())
}
