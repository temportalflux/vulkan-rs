use engine::{utility::VoidResult, Application};
pub use temportal_engine as engine;

#[path = "graphics/_.rs"]
mod graphics;

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

	use engine::ui::prelude::*;

	let mut ui_app = Application::new();
	ui_app.setup(widget::setup);
	let atlas_mapping = std::collections::HashMap::new();
	let image_sizes = std::collections::HashMap::new();
	let mut ui_renderer = TesselateRenderer::new(
		TesselationVerticesFormat::Interleaved,
		(),
		&atlas_mapping,
		&image_sizes,
	);

	let tree = widget! {
		(text_box: { Props::new(TextBoxProps {
			text: "Hello World!".to_owned(),
			color: utils::Color {
				r: 1.0,
				g: 1.0,
				b: 1.0,
				a: 1.0,
			},
			font: TextBoxFont {
				name: "unispace".to_owned(),
				size: 60.0,
			},
			.. Default::default()
		}) })
	};

	ui_app.apply(tree);

	let mapping = CoordsMapping::new(Rect {
		left: 0.0,
		right: 1280.0,
		top: 0.0,
		bottom: 720.0,
	});
	ui_app.process();
	let _res = ui_app.layout(&mapping, &mut DefaultLayoutEngine);
	if let Ok(output) = ui_app.render(&mapping, &mut ui_renderer) {
		log::debug!("{:?}", output);
	}

	// TODO: create a non-default renderpass info which has multiple subpasses (one for world, and at least one more for just ui)
	let chain = window.create_render_chain(engine::graphics::renderpass::Info::default())?;
	let _ui_render = graphics::UIRender::new(&chain);

	engine.run(chain);
	window.wait_until_idle().unwrap();
	Ok(())
}
