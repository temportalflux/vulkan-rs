use engine::{
	ecs::{Builder, WorldExt},
	math::{vector, Quaternion, Vector},
	utility::VoidResult,
	world, Application,
};
pub use temportal_engine as engine;
use std::sync::{Arc, RwLock};

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
	let mut engine = engine::Engine::new::<BoidDemo>()?;

	let mut world = ecs::World::new();
	world.register::<ecs::components::Position2D>();
	world.register::<ecs::components::Velocity2D>();
	world.register::<ecs::components::Orientation>();
	world.insert(ecs::resources::DeltaTime(std::time::Duration::default()));

	let mut window = engine::window::Window::builder()
		.with_title(BoidDemo::display_name())
		.with_size(1000.0, 1000.0)
		.with_resizable(true)
		.with_application::<BoidDemo>()
		.with_clear_color(Vector::new([
			0.0, 0.25, 0.5, 1.0,
		]))
		.build(&engine)?;
	window.create_render_chain(engine::graphics::renderpass::Info::default())?;

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
				graphics::RenderBoids::new(&window.render_chain(), camera_view_space)?,
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

	let ecs_context = Arc::new(RwLock::new(ecs::Context {
		world, dispatcher
	}));
	engine.add_system(&ecs_context);
	
	engine.run(window.render_chain().clone());
	window.wait_until_idle().unwrap();
	Ok(())
}
