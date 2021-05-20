use engine::{
	math::{vector, Vector},
	ui,
	utility::VoidResult,
	Application,
};
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
			system.add_shader(
				SystemShader::TextVertex,
				&UIDemo::get_asset_id("shaders/ui/text/vertex"),
			)?;
			system.add_shader(
				SystemShader::TextFragment,
				&UIDemo::get_asset_id("shaders/ui/text/fragment"),
			)?;
			system.add_shader(
				SystemShader::MeshVertex,
				&UIDemo::get_asset_id("shaders/ui/mesh/vertex"),
			)?;
			system.add_shader(
				SystemShader::MeshSimpleFragment,
				&UIDemo::get_asset_id("shaders/ui/mesh/simple_fragment"),
			)?;
			system.add_shader(
				SystemShader::MeshImageFragment,
				&UIDemo::get_asset_id("shaders/ui/mesh/image_fragment"),
			)?;
			system.add_font(
				&UIDemo::get_asset_id("font/unispace"),
				Arc::new(|font_size| -> Vector<f32, 2> {
					match font_size {
						f if f >= 100.0 => vector![0.8, 0.09],
						_ => vector![0.78, 0.08],
					}
				}),
			)?;
			system.add_texture(&UIDemo::get_asset_id("textures/background"))?;

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
							text: "item 2\nand another line".to_owned(),
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
					(vertical_box [
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
						(#{"row2colum1"} image_box: { Props::new(ImageBoxProps {
							width: ImageBoxSizeValue::Fill,
							height: ImageBoxSizeValue::Fill,
							content_keep_aspect_ratio: None,
							material: ImageBoxMaterial::Color(ImageBoxColor {
								color: Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 },
								scaling: ImageBoxImageScaling::Stretch,
							}),
							transform: Transform {
								pivot: Vec2 { x: 0.0, y: 0.0 },
								align: Vec2 { x: 0.0, y: 0.0 },
								translation: Vec2 { x: 0.0, y: 0.0 },
								rotation: 0.0,
								scale: Vec2 { x: 1.0, y: 1.0 },
								skew: Vec2 { x: 0.0, y: 0.0 },
							},
						}) })
						(#{"row3colum1"} image_box: { Props::new(ImageBoxProps {
							width: ImageBoxSizeValue::Fill,
							height: ImageBoxSizeValue::Fill,
							content_keep_aspect_ratio: None,
							material: ImageBoxMaterial::Image(ImageBoxImage {
								id: "textures/background".to_owned(),
								source_rect: None,
								scaling: ImageBoxImageScaling::Stretch,
								tint: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
							}),
							transform: Transform {
								pivot: Vec2 { x: 0.0, y: 0.0 },
								align: Vec2 { x: 0.0, y: 0.0 },
								translation: Vec2 { x: 0.0, y: 0.0 },
								rotation: 0.0,
								scale: Vec2 { x: 1.0, y: 1.0 },
								skew: Vec2 { x: 0.0, y: 0.0 },
							},
						}) })
					])
				])
			});

			system
		}));
		engine.add_system(&ui_system);
		chain
			.write()
			.unwrap()
			.add_render_chain_element(&ui_system)?;
	}

	engine.run(chain.clone());
	Ok(())
}
