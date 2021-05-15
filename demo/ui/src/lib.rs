use engine::{ui, utility::VoidResult, Application};
use std::sync::{Arc, RwLock};
pub use temportal_engine as engine;

pub struct UIDemo();
impl Application for UIDemo {
	fn name() -> &'static str {
		std::env!("CARGO_PKG_NAME")
	}
	fn display_name() -> &'static str {
		"(RA)UI Demo"
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
	let mut engine = engine::Engine::new::<UIDemo>()?;

	let mut window = engine::window::Window::builder()
		.with_title(UIDemo::display_name())
		.with_size(1280.0, 720.0)
		.with_resizable(true)
		.with_application::<UIDemo>()
		.build(&engine)?;

	let chain = window.create_render_chain(engine::graphics::renderpass::Info::default())?;

	{
		let ui_system = Arc::new(RwLock::new({
			use ui::*;
			let mut system = ui::System::new(&chain.read().unwrap())?;
			system.add_text_shader(&UIDemo::get_asset_id("shaders/text/vertex"))?;
			system.add_text_shader(&UIDemo::get_asset_id("shaders/text/fragment"))?;
			system.add_font(&UIDemo::get_asset_id("font/unispace"))?;

			system.apply_tree(widget! {
				(horizontal_box [
					(vertical_box [
						(#{"hello_world"} text_box: { Props::new(TextBoxProps {
							text: "Hello World!".to_owned(),
							color: utils::Color {
								r: 0.0,
								g: 1.0,
								b: 1.0,
								a: 1.0,
							},
							font: TextBoxFont {
								name: "unispace".to_owned(),
								size: 100.0,
							},
							.. Default::default()
						}) })
						(#{"item2"} text_box: { Props::new(TextBoxProps {
							text: "item 2".to_owned(),
							color: utils::Color {
								r: 1.0,
								g: 0.0,
								b: 0.0,
								a: 1.0,
							},
							font: TextBoxFont {
								name: "unispace".to_owned(),
								size: 20.0,
							},
							.. Default::default()
						}) })
						(#{"item3"} text_box: { Props::new(TextBoxProps {
							text: "fdsa".to_owned(),
							color: utils::Color {
								r: 1.0,
								g: 1.0,
								b: 1.0,
								a: 1.0,
							},
							font: TextBoxFont {
								name: "unispace".to_owned(),
								size: 20.0,
							},
							.. Default::default()
						}) })
					])
					(#{"row1column1"} text_box: { Props::new(TextBoxProps {
						text: "C".to_owned(),
						color: utils::Color {
							r: 0.0,
							g: 0.0,
							b: 1.0,
							a: 1.0,
						},
						font: TextBoxFont {
							name: "unispace".to_owned(),
							size: 30.0,
						},
						.. Default::default()
					}) })
				])
			});

			system.set_resolution(engine::math::vector![1280, 720]);

			system
		}));
		engine.add_system(&ui_system);
		let mut chain = chain.write().unwrap();
		chain.add_render_chain_element(&ui_system)?;
		chain.add_command_recorder(&ui_system)?;
	}

	engine.run(chain.clone());
	Ok(())
}
