use engine::{
	display,
	ecs::{Builder, WorldExt},
	math::{vector, Vector},
	utility::{AnyError, VoidResult},
	Engine,
};
use std::{cell::RefCell, rc::Rc};
pub use temportal_engine as engine;

#[path = "graphics/_.rs"]
pub mod graphics;

#[path = "ecs/_.rs"]
pub mod ecs;

pub fn name() -> &'static str {
	std::env!("CARGO_PKG_NAME")
}

pub fn create_engine() -> Result<Rc<RefCell<Engine>>, AnyError> {
	let engine = Engine::new()?;
	engine
		.borrow_mut()
		.set_application("Boids", temportal_engine::utility::make_version(0, 1, 0));
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

pub fn run() -> VoidResult {
	engine::logging::init(name())?;
	let engine = create_engine()?;

	let mut world = ecs::World::new();
	world.register::<ecs::components::Position2D>();
	world.register::<ecs::components::Orientation>();
	world.register::<ecs::components::BoidRender>();

	let display = Engine::create_display_manager(&engine)?;
	let window = display::WindowBuilder::default()
		.title(engine.borrow().app_info().app_name())
		.size(1000, 1000)
		.constraints(vulkan_device_constraints())
		.build(&mut display.borrow_mut())?;
	let render_chain = window
		.borrow()
		.create_render_chain(&mut display.borrow_mut(), create_render_pass_info())?;
	render_chain
		.borrow_mut()
		.add_clear_value(graphics::renderpass::ClearValue::Color(Vector::new([
			0.0, 0.25, 0.5, 1.0,
		])));

	let _render_boids =
		graphics::RenderBoids::new(&engine.borrow(), &mut render_chain.borrow_mut())?;

	let mut dispatcher = ecs::DispatcherBuilder::new()
		.with(ecs::systems::InstanceCollector::new(), "render-instance-collector", &[])
		.build();

	world
		.create_entity()
		.with(ecs::components::Position2D(vector![0.0, 0.0]))
		.with(ecs::components::BoidRender::new(vector![0.5, 0.0, 1.0, 1.0]))
		.build();

	while !display.borrow().should_quit() {
		display.borrow_mut().poll_all_events()?;
		dispatcher.dispatch(&mut world);
		render_chain.borrow_mut().render_frame()?;
	}
	render_chain.borrow().logical().wait_until_idle()?;

	Ok(())
}

fn vulkan_device_constraints() -> Vec<graphics::device::physical::Constraint> {
	use graphics::{
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

fn create_render_pass_info() -> graphics::renderpass::Info {
	use graphics::{flags, renderpass};
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
