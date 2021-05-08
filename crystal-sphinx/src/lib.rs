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

	let mut ui_app = engine::ui::core::application::Application::new();
	let atlas_mapping = std::collections::HashMap::new();
	let image_sizes = std::collections::HashMap::new();
	let mut ui_renderer = engine::ui::renderer::tesselate::renderer::TesselateRenderer::new(
		engine::ui::prelude::TesselationVerticesFormat::Interleaved,
		(),
		&atlas_mapping,
		&image_sizes,
	);

	let tree = engine::ui::core::widget! {
		(engine::ui::core::widget::component::containers::vertical_box::vertical_box [
			(#{"hi"} engine::ui::core::widget::component::interactive::button::button: { "Say hi".to_owned() })
		])
	};

	ui_app.apply(tree);

	let mapping = engine::ui::core::layout::CoordsMapping::new(engine::ui::prelude::Rect {
		left: 0.0,
		right: 1280.0,
		top: 0.0,
		bottom: 720.0,
	});
	if let Ok(output) = ui_app.render(&mapping, &mut ui_renderer) {
		log::debug!("{:?}", output);
	}

	let chain = window.create_render_chain(engine::graphics::renderpass::Info::default())?;
	let _text_render = TextRender::new(&chain);

	engine.run(chain);
	window.wait_until_idle().unwrap();
	Ok(())
}
