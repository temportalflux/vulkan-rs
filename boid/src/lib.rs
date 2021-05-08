use engine::{
	asset, display,
	ecs::{Builder, WorldExt},
	math::{vector, Quaternion, Vector},
	utility::VoidResult,
	world, Application,
};
pub use temportal_engine as engine;

#[path = "graphics/_.rs"]
pub mod graphics;

#[path = "ecs/_.rs"]
pub mod ecs;

pub struct BoidDemo();
impl Application for BoidDemo {
	fn name() -> &'static str {
		std::env!("CARGO_PKG_NAME")
	}
	fn display_name() -> &'static str {
		"Boids"
	}
	fn location() -> &'static str {
		std::env!("CARGO_MANIFEST_DIR")
	}
	fn version() -> u32 {
		temportal_engine::utility::make_version(0, 1, 0)
	}
}

pub fn run() -> VoidResult {
	engine::logging::init::<BoidDemo>()?;
	let task_watcher = engine::task::initialize_system();
	engine::register_asset_types();
	asset::Library::scan_application::<BoidDemo>()?;

	let mut world = ecs::World::new();
	world.register::<ecs::components::Position2D>();
	world.register::<ecs::components::Velocity2D>();
	world.register::<ecs::components::Orientation>();
	world.insert(ecs::resources::DeltaTime(std::time::Duration::default()));

	let mut display = engine::display::Manager::new()?;
	let window = display::WindowBuilder::default()
		.with_application::<BoidDemo>()
		.title(BoidDemo::display_name())
		.size(1000, 1000)
		.constraints(vulkan_device_constraints())
		.build(&mut display)?;
	let render_chain = window.create_render_chain(create_render_pass_info())?;
	render_chain
		.write()
		.unwrap()
		.add_clear_value(graphics::renderpass::ClearValue::Color(Vector::new([
			0.0, 0.25, 0.5, 1.0,
		])));

	let camera_view_space = vector![-15.0, 15.0, -15.0, 15.0];
	let wrapping_world_bounds_min = vector![-15.0, -15.0];
	let wrapping_world_bounds_max = vector![15.0, 15.0];

	let mut dispatcher = ecs::DispatcherBuilder::new()
		.with(ecs::systems::MoveEntities::default(), "move_entities", &[])
		.with(
			ecs::systems::WorldBounds::default()
				.with_bounds(wrapping_world_bounds_min, wrapping_world_bounds_max),
			"world_bounds",
			&["move_entities"],
		)
		.with(
			ecs::systems::InstanceCollector::new(
				graphics::RenderBoids::new(&render_chain, camera_view_space)?,
				100,
			),
			"render-instance-collector",
			&["world_bounds"],
		)
		.build();

	dispatcher.setup(&mut world);

	for y in -5..5 {
		for x in -5..5 {
			let frag = ((((y + 5) * 11) + (x + 5)) as f32) / (11.0 * 11.0);
			world
				.create_entity()
				.with(ecs::components::Position2D(vector![
					x as f32 * 3.0,
					y as f32 * 3.0 + 1.0
				]))
				.with(ecs::components::Orientation(Quaternion::from_axis_angle(
					-world::global_forward(),
					360.0_f32.to_radians() * frag,
				)))
				.with(ecs::components::Velocity2D(vector![2.0, 2.0]))
				.with(ecs::components::BoidRender::new(vector![
					((x + 5) as f32) / 11.0,
					0.0,
					((y + 5) as f32) / 11.0,
					1.0
				]))
				.build();
		}
	}

	let mut prev_frame_time = std::time::Instant::now();
	while !display.should_quit() {
		let frame_time = std::time::Instant::now();
		{
			let mut delta_time = world.write_resource::<ecs::resources::DeltaTime>();
			*delta_time = ecs::resources::DeltaTime(frame_time - prev_frame_time);
		}
		display.poll_all_events()?;
		task_watcher.poll();
		dispatcher.dispatch(&mut world);
		render_chain.write().unwrap().render_frame()?;
		prev_frame_time = frame_time;
	}
	task_watcher.poll_until_empty();
	render_chain.read().unwrap().logical().wait_until_idle()?;

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
