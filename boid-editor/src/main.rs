use boid;
use engine::utility::VoidResult;
use temportal_engine as engine;
use temportal_engine_editor as editor;

fn main() -> VoidResult {
	engine::logging::init(boid::name())?;

	let editor = editor::Editor::new(boid::create_engine()?, boid::name())?;
	if editor.borrow().run_commandlets()? {
		return Ok(());
	}

	Ok(())
}
