use std::{cell::RefCell, rc::Rc};
use temportal_engine::{self, display, Engine};
use temportal_graphics::{device::physical, flags};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let engine = crate_engine()?;
	{
		let engine_mut = engine.borrow_mut();
		if engine_mut.is_build_instance() {
			return engine_mut.build();
		}
	}

	let mut display = Engine::create_display_manager(&engine)?;
	let mut window = create_window(&mut display, "Triangle Demo", 800, 600)?;

	temportal_engine::run(
		&engine,
		&mut display,
		&mut window,
		include_bytes!("triangle.vert.spirv").to_vec(),
		include_bytes!("triangle.frag.spirv").to_vec(),
	)
}

fn crate_engine() -> Result<Rc<RefCell<Engine>>, Box<dyn std::error::Error>> {
	let mut engine = Engine::new()?
		.set_application("Triangle", temportal_engine::utility::make_version(0, 1, 0));
	engine.build_assets_callback = Some(build_assets);
	Ok(Rc::new(RefCell::new(engine)))
}

fn create_window(
	display: &mut display::Manager,
	name: &str,
	width: u32,
	height: u32,
) -> Result<Rc<RefCell<display::Window>>, Box<dyn std::error::Error>> {
	let window = display.create_window(name, width, height)?;
	{
		let mut mut_window = window.borrow_mut();
		mut_window.find_physical_device(&mut vulkan_device_constraints())?;
		mut_window.create_logical()?;
		mut_window.create_render_chain()?;
	}
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