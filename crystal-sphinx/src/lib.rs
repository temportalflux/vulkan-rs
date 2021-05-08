use engine::{utility::VoidResult, Application};
pub use temportal_engine as engine;

#[path = "graphics/_.rs"]
mod graphics;
use graphics::TextRender;

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
		temportal_engine::utility::make_version(0, 1, 0)
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

	let chain = window.create_render_chain(engine::graphics::renderpass::Info::default())?;
	let _text_render = TextRender::new(&chain);

	engine.run(chain);
	window.wait_until_idle().unwrap();
	Ok(())
}
