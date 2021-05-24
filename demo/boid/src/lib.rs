use engine::{
	math::{vector, Vector},
	utility::VoidResult,
	Application,
};
use std::sync::{Arc, RwLock};
pub use temportal_engine as engine;

#[path = "graphics/_.rs"]
pub mod graphics;

#[path = "ecs/_.rs"]
pub mod ecs;

#[path = "ui/_.rs"]
pub mod ui;

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
		engine::utility::make_version(
			std::env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
			std::env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
			std::env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
		)
	}
}

pub fn run() -> VoidResult {
	let mut engine = engine::Engine::new::<BoidDemo>()?;

	let camera_view_space = vector![-15.0, 15.0, -15.0, 15.0];
	let wrapping_world_bounds_min = vector![-15.0, -15.0];
	let wrapping_world_bounds_max = vector![15.0, 15.0];

	let mut ecs_context = ecs::Context::new()
		.with_component::<ecs::components::Position2D>()
		.with_component::<ecs::components::Velocity2D>()
		.with_component::<ecs::components::Orientation>()
		.with_component::<ecs::components::ai::Wander2D>()
		.with_system(ecs::systems::ai::WanderIn2D::default())
		.with_system(ecs::systems::MoveEntities::default())
		.with_system(
			ecs::systems::WorldBounds::default()
				.with_bounds(wrapping_world_bounds_min, wrapping_world_bounds_max),
		)
		.with_system(ecs::systems::InputCreateEntity::default())
		.with_system(ecs::systems::InputDestroyEntity::default());

	let mut window = engine::window::Window::builder()
		.with_title(BoidDemo::display_name())
		.with_size(1000.0, 1000.0)
		.with_resizable(true)
		.with_application::<BoidDemo>()
		.with_clear_color(Vector::new([0.0, 0.25, 0.5, 1.0]))
		.build(&engine)?;
	let chain = window.create_render_chain(engine::graphics::renderpass::Info::default())?;

	ecs_context.add_system(ecs::systems::InstanceCollector::new(
		graphics::RenderBoids::new(&window.render_chain(), camera_view_space)?,
		100,
	));

	ecs_context.setup();
	let ecs_context = Arc::new(RwLock::new(ecs_context));
	engine.add_system(&ecs_context);

	engine::ui::System::new(&chain)?
		.with_engine_shaders()?
		.with_all_fonts()?
		.with_tree_root(engine::ui::make_widget!(crate::ui::root))
		.attach_system(&mut engine, &chain)?;

	engine.run(chain.clone());
	Ok(())
}
