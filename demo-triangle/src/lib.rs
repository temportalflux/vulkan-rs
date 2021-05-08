use engine::{
	asset, display,
	graphics::{device::physical, flags, renderpass},
	math::Vector,
	utility::VoidResult,
};
pub use temportal_engine as engine;

#[path = "renderer.rs"]
mod renderer;
#[path = "vertex.rs"]
mod vertex;
pub use vertex::*;

pub fn name() -> &'static str {
	std::env!("CARGO_PKG_NAME")
}

fn scan_assets() -> VoidResult {
	let mut library = asset::Library::get().write().unwrap();
	library.scan_pak(
		&[
			std::env!("CARGO_MANIFEST_DIR"),
			format!("{}.pak", name()).as_str(),
		]
		.iter()
		.collect::<std::path::PathBuf>(),
	)
}

pub fn run(log_name: &str) -> VoidResult {
	engine::logging::init(log_name)?;
	engine::register_asset_types();
	scan_assets()?;
	let (task_spawner, task_watcher) = engine::task::create_system();

	let mut display = display::Manager::new()?;
	let window = display::WindowBuilder::default()
		.with_info(
			engine::make_app_info()
				.with_application("Triangle", temportal_engine::utility::make_version(0, 1, 0)),
		)
		.title("Triangle Demo")
		.size(800, 600)
		.constraints(vulkan_device_constraints())
		.resizable(true)
		.build(&mut display)?;
	let render_chain = window
		.borrow()
		.create_render_chain(create_render_pass_info(), task_spawner.clone())?;
	render_chain
		.write()
		.unwrap()
		.add_clear_value(renderpass::ClearValue::Color(Vector::new([
			0.0, 0.0, 0.0, 1.0,
		])));

	let _renderer = renderer::Triangle::new(&render_chain);

	while !display.should_quit() {
		display.poll_all_events()?;
		task_watcher.poll();
		render_chain.write().unwrap().render_frame()?;
	}
	task_watcher.poll_until_empty();
	render_chain.read().unwrap().logical().wait_until_idle()?;

	Ok(())
}

fn vulkan_device_constraints() -> Vec<physical::Constraint> {
	use physical::Constraint::*;
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
