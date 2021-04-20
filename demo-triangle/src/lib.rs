use std::{cell::RefCell, rc::Rc};
use temportal_engine::{self, Engine};

pub fn create_engine() -> Result<Rc<RefCell<Engine>>, Box<dyn std::error::Error>> {
	let engine = Engine::new()?;
	engine
		.borrow_mut()
		.set_application("Triangle", temportal_engine::utility::make_version(0, 1, 0));
	Ok(engine)
}
