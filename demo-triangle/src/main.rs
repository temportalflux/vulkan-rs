use std::rc::Rc;
use temportal_engine::{self, display, Engine};
use temportal_graphics::{device::physical, flags};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let engine = crate_engine()?;
	if engine.is_build_instance() {
		return engine.build();
	}

	let display = Engine::create_display_manager(&engine)?;
	let mut window = create_window(&display, "Triangle Demo", 800, 600)?;

	temportal_engine::run(
		&display,
		&mut window,
		include_bytes!("triangle.vert.spirv").to_vec(),
		include_bytes!("triangle.frag.spirv").to_vec(),
	)
}

fn crate_engine() -> Result<Rc<Engine>, Box<dyn std::error::Error>> {
	let mut engine = temportal_engine::init()?
		.set_application("Triangle", temportal_engine::utility::make_version(0, 1, 0));
	engine.build_assets_callback = Some(build_assets);
	Ok(Rc::new(engine))
}

fn create_window(
	display: &display::Manager,
	name: &str,
	width: u32,
	height: u32,
) -> Result<display::Window, Box<dyn std::error::Error>> {
	let mut window = display.create_window(name, width, height)?;
	window.find_physical_device(&mut vulkan_device_constraints())?;
	window.create_logical()?;
	window.create_render_chain()?;
	Ok(window)
}

fn vulkan_device_constraints() -> Vec<physical::Constraint> {
	use physical::Constraint::*;
	vec![
		HasSurfaceFormats(
			flags::Format::B8G8R8A8_SRGB,
			flags::ColorSpace::SRGB_NONLINEAR_KHR,
		),
		HasExtension(String::from("VK_KHR_swapchain")),
		PrioritizedSet(
			vec![
				CanPresentWith(flags::PresentMode::MAILBOX_KHR, Some(1)),
				CanPresentWith(flags::PresentMode::FIFO_KHR, None),
			],
			false,
		),
		PrioritizedSet(
			vec![
				IsDeviceType(physical::Kind::DISCRETE_GPU, Some(100)),
				IsDeviceType(physical::Kind::INTEGRATED_GPU, Some(0)),
			],
			false,
		),
	]
}

fn build_assets(
	ctx: &mut temportal_engine::build::BuildContext,
) -> Result<(), Box<dyn std::error::Error>> {
	let options = ctx.shader.make_options();

	let outpath = temportal_engine::build::get_output_dir("demo-triangle")?;

	ctx.shader.compile_into_spirv(
		&options,
		&outpath,
		temportal_engine::build::Shader {
			name: String::from("triangle.vert"),
			source: String::from(include_str!("triangle.vert")),
			kind: temportal_engine::build::ShaderKind::Vertex,
			entry_point: String::from("main"),
		},
	)?;

	ctx.shader.compile_into_spirv(
		&options,
		&outpath,
		temportal_engine::build::Shader {
			name: String::from("triangle.frag"),
			source: String::from(include_str!("triangle.frag")),
			kind: temportal_engine::build::ShaderKind::Fragment,
			entry_point: String::from("main"),
		},
	)?;

	Ok(())
}
