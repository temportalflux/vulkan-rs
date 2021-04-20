use engine::{display, Engine};
use std::{cell::RefCell, rc::Rc};
use temportal_engine as engine;
use temportal_graphics::{device::physical, flags, renderpass};
use temportal_math::Vector;

#[path = "render/TriangleRenderer.rs"]
mod renderer;
use renderer::TriangleRenderer;

#[path = "lib.rs"]
mod lib;
use lib::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let engine = crate::create_engine()?;

	let display = Engine::create_display_manager(&engine)?;
	let window = create_window(&mut display.borrow_mut(), "Triangle Demo", 800, 600)?;

	let vert_bytes: Vec<u8>;
	let frag_bytes: Vec<u8>;
	{
		let eng_ref = engine.borrow();
		{
			let path = [
				std::env!("CARGO_MANIFEST_DIR"),
				"output",
				"triangle_vert.bin",
			]
			.iter()
			.collect::<std::path::PathBuf>();
			let asset = eng_ref
				.assets
				.loader
				.decompile(&eng_ref.assets.types, &path)?;
			let shader = engine::asset::as_asset::<engine::graphics::Shader>(&asset);
			vert_bytes = shader.contents().clone();
		}
		{
			let path = [
				std::env!("CARGO_MANIFEST_DIR"),
				"output",
				"triangle_frag.bin",
			]
			.iter()
			.collect::<std::path::PathBuf>();
			let asset = eng_ref
				.assets
				.loader
				.decompile(&eng_ref.assets.types, &path)?;
			let shader = engine::asset::as_asset::<engine::graphics::Shader>(&asset);
			frag_bytes = shader.contents().clone();
		}
	}

	let renderer = Rc::new(RefCell::new(TriangleRenderer::new(vert_bytes, frag_bytes)));
	{
		let mut window_mut = window.borrow_mut();
		window_mut.add_render_chain_element(renderer.clone())?;
		window_mut.add_command_recorder(renderer.clone())?;
	}

	window.borrow_mut().create_render_chain()?;

	{
		let mut window_mut = window.borrow_mut();
		window_mut.add_clear_value(renderpass::ClearValue::Color(Vector::new([
			0.0, 0.0, 0.0, 1.0,
		])));
		window_mut.mark_commands_dirty();
	}

	while !display.borrow().should_quit() {
		display.borrow_mut().poll_all_events()?;
		window.borrow_mut().render_frame()?;
		::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
	}

	window.borrow().logical().wait_until_idle()?;

	Ok(())
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
		mut_window.create_render_pass(create_render_pass_info())?;
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

fn create_render_pass_info() -> renderpass::Info {
	let mut rp_info = renderpass::Info::default();

	let frame_attachment_index = rp_info.attach(
		renderpass::Attachment::default()
			.set_format(flags::Format::B8G8R8A8_SRGB)
			.set_sample_count(flags::SampleCount::_1)
			.set_general_ops(renderpass::AttachmentOps {
				load: flags::AttachmentLoadOp::CLEAR,
				store: flags::AttachmentStoreOp::STORE,
			})
			.set_final_layout(flags::ImageLayout::PRESENT_SRC_KHR),
	);

	let main_pass_index = rp_info.add_subpass(renderpass::Subpass::default().add_attachment_ref(
		frame_attachment_index,
		flags::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
	));

	rp_info.add_dependency(
		renderpass::Dependency::new(None).set_stage(flags::PipelineStage::COLOR_ATTACHMENT_OUTPUT),
		renderpass::Dependency::new(Some(main_pass_index))
			.set_stage(flags::PipelineStage::COLOR_ATTACHMENT_OUTPUT)
			.set_access(flags::Access::COLOR_ATTACHMENT_WRITE),
	);

	rp_info
}
