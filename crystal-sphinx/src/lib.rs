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
		.constraints(vulkan_device_constraints())
		.resizable(true)
		.build(&mut display)?;
	let render_chain = window.create_render_chain(create_render_pass_info())?;
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

fn vulkan_device_constraints() -> Vec<engine::graphics::device::physical::Constraint> {
	use engine::graphics::{
		device::physical::{Constraint::*, Kind},
		flags,
	};
	vec![
		HasSurfaceFormats(
			flags::Format::B8G8R8A8_SRGB,
			flags::ColorSpace::SRGB_NONLINEAR,
		),
		HasExtension(String::from("VK_KHR_swapchain")),
		PrioritizedSet(
			vec![
				CanPresentWith(flags::PresentMode::MAILBOX, Some(1)),
				CanPresentWith(flags::PresentMode::FIFO, None),
			],
			false,
		),
		PrioritizedSet(
			vec![
				IsDeviceType(Kind::DISCRETE_GPU, Some(100)),
				IsDeviceType(Kind::INTEGRATED_GPU, Some(0)),
			],
			false,
		),
	]
}

fn create_render_pass_info() -> engine::graphics::renderpass::Info {
	use engine::graphics::{flags, renderpass};
	let mut rp_info = renderpass::Info::default();

	let frame_attachment_index = rp_info.attach(
		renderpass::Attachment::default()
			.set_format(flags::Format::B8G8R8A8_SRGB)
			.set_sample_count(flags::SampleCount::TYPE_1)
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
