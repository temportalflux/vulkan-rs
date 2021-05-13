use engine::{utility::VoidResult, Application};
pub use temportal_engine as engine;

pub struct CrystalSphinx();
impl Application for CrystalSphinx {
	fn name() -> &'static str {
		std::env!("CARGO_PKG_NAME")
	}
	fn display_name() -> &'static str {
		"Crystal Sphinx"
	}
	fn location() -> &'static str {
		std::env!("CARGO_MANIFEST_DIR")
	}
	fn version() -> u32 {
		engine::utility::make_version(
			std::env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
			std::env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
			std::env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
		)
	}
}

pub fn run() -> VoidResult {
	let engine = engine::Engine::new::<CrystalSphinx>()?;

	let mut window = engine::window::Window::builder()
		.with_title(CrystalSphinx::display_name())
		.with_size(1280.0, 720.0)
		.with_resizable(true)
		.with_application::<CrystalSphinx>()
		.build(&engine)?;

	// TODO: create a non-default renderpass info which has multiple subpasses (one for world, and at least one more for just ui)
	let chain = window.create_render_chain(engine::graphics::renderpass::Info::default())?;

	engine.run(chain);
	window.wait_until_idle().unwrap();
	Ok(())
}
