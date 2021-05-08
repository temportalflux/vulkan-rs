use engine::{asset, display, math::Vector, utility::VoidResult, Application};
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
	engine::logging::init::<CrystalSphinx>()?;
	let task_watcher = engine::task::initialize_system();
	engine::register_asset_types();
	asset::Library::scan_application::<CrystalSphinx>()?;

	let mut display = engine::display::Manager::new()?;
	let window = display::WindowBuilder::default()
		.with_application::<CrystalSphinx>()
		.title(CrystalSphinx::display_name())
		.size(1280, 720)
		.constraints(engine::graphics::device::physical::default_constraints())
		.resizable(true)
		.build(&mut display)?;
	let render_chain = window.create_render_chain(engine::graphics::renderpass::Info::default())?;
	render_chain
		.write()
		.unwrap()
		.add_clear_value(engine::graphics::renderpass::ClearValue::Color(
			Vector::new([0.0, 0.0, 0.0, 1.0]),
		));

	let _text_render = TextRender::new(&render_chain);

	while !display.should_quit() {
		display.poll_all_events()?;
		task_watcher.poll();
		render_chain.write().unwrap().render_frame()?;
	}
	task_watcher.poll_until_empty();
	render_chain.read().unwrap().logical().wait_until_idle()?;

	Ok(())
}
