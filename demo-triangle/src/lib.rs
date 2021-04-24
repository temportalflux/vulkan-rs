use std::{cell::RefCell, rc::Rc};

use engine::{
	display,
	graphics::{device::physical, flags, renderpass},
	math::Vector,
	utility::{AnyError, VoidResult},
	Engine,
};
pub use temportal_engine as engine;

#[path = "render/TriangleRenderer.rs"]
mod renderer;
use renderer::TriangleRenderer;

pub fn name() -> &'static str {
	std::env!("CARGO_PKG_NAME")
}

pub fn create_engine() -> Result<Rc<RefCell<Engine>>, AnyError> {
	let engine = Engine::new()?;
	engine
		.borrow_mut()
		.set_application("Triangle", temportal_engine::utility::make_version(0, 1, 0));
	scan_assets(&mut engine.borrow_mut())?;
	Ok(engine)
}

fn scan_assets(engine: &mut Engine) -> VoidResult {
	let pak_path = [
		std::env!("CARGO_MANIFEST_DIR"),
		format!("{}.pak", name()).as_str(),
	]
	.iter()
	.collect::<std::path::PathBuf>();
	engine.assets.library.scan_pak(&pak_path)
}

pub fn run(log_name: &str) -> VoidResult {
	engine::logging::init(log_name)?;
	let engine = create_engine()?;

	let display = Engine::create_display_manager(&engine)?;
	let window = display::WindowBuilder::default()
		.title("Triangle Demo")
		.size(800, 600)
		.constraints(vulkan_device_constraints())
		.build(&mut display.borrow_mut())?;
	let render_chain = window
		.borrow()
		.create_render_chain(&mut display.borrow_mut(), create_render_pass_info())?;
	render_chain
		.borrow_mut()
		.add_clear_value(renderpass::ClearValue::Color(Vector::new([
			0.0, 0.0, 0.0, 1.0,
		])));

	let _renderer = TriangleRenderer::new(&engine.borrow(), &mut render_chain.borrow_mut());

	while !display.borrow().should_quit() {
		display.borrow_mut().poll_all_events()?;
		render_chain.borrow_mut().render_frame()?;
	}
	render_chain.borrow().logical().wait_until_idle()?;

	Ok(())
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
