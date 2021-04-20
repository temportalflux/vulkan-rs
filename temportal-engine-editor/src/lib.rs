extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;
extern crate shaderc;

use std::{
	cell::RefCell,
	rc::{Rc, Weak},
};
use temportal_engine as engine;

#[path = "asset/_.rs"]
pub mod asset;

#[path = "ui/_.rs"]
pub mod ui;

use std::time::Instant;

pub struct Editor {
	ui_elements: Vec<Weak<RefCell<dyn ui::Element>>>,
	last_frame: Instant,
	imgui_renderer: Option<imgui_opengl_renderer::Renderer>,
	imgui_win: Option<imgui_sdl2::ImguiSdl2>,
	imgui_ctx: Option<imgui::Context>,
	_gl_context: Option<sdl2::video::GLContext>,
	sdl_window: Option<sdl2::video::Window>,
	display: Option<Rc<RefCell<engine::display::Manager>>>,
	asset_manager: asset::Manager,
	engine: Weak<RefCell<engine::Engine>>,
}

impl Editor {
	pub fn new(engine: &Rc<RefCell<engine::Engine>>) -> Rc<RefCell<Editor>> {
		let mut editor = Editor {
			engine: Rc::downgrade(&engine),
			asset_manager: asset::Manager::new(),
			display: None,
			sdl_window: None,
			_gl_context: None,
			imgui_ctx: None,
			imgui_win: None,
			imgui_renderer: None,
			last_frame: Instant::now(),
			ui_elements: Vec::new(),
		};
		editor
			.asset_manager()
			.register::<engine::graphics::Shader>(asset::ShaderEditorMetadata::boxed());
		Rc::new(RefCell::new(editor))
	}

	pub fn asset_manager(&mut self) -> &mut asset::Manager {
		&mut self.asset_manager
	}

	pub fn init_display(&mut self) -> engine::utility::Result<()> {
		let engine = self.engine.upgrade().unwrap();
		self.display = Some(engine::Engine::create_display_manager(&engine)?);
		let video = self.display().borrow().video_subsystem()?;
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		gl_attr.set_context_version(3, 0);
		Ok(())
	}

	pub fn display(&self) -> &Rc<RefCell<engine::display::Manager>> {
		&self.display.as_ref().unwrap()
	}

	pub fn create_window(
		&mut self,
		title: &str,
		width: u32,
		height: u32,
	) -> engine::utility::Result<()> {
		let video = self.display().borrow().video_subsystem()?;
		let window = engine::utility::as_window_error(
			video
				.window(title, width, height)
				.position_centered()
				.resizable()
				.opengl()
				.allow_highdpi()
				.build(),
		)?;

		self._gl_context = Some(
			window
				.gl_create_context()
				.expect("Couldn't create GL context"),
		);
		gl::load_with(|s| video.gl_get_proc_address(s) as _);

		let mut ctx = imgui::Context::create();
		ctx.set_ini_filename(None);

		self.imgui_win = Some(imgui_sdl2::ImguiSdl2::new(&mut ctx, &window));

		self.imgui_renderer = Some(imgui_opengl_renderer::Renderer::new(&mut ctx, |s| {
			video.gl_get_proc_address(s) as _
		}));

		self.imgui_ctx = Some(ctx);
		self.sdl_window = Some(window);

		Ok(())
	}
}

impl engine::display::EventListener for Editor {
	fn on_event(&mut self, event: &sdl2::event::Event) -> bool {
		let mut imctx = self.imgui_ctx.as_mut().unwrap();
		let imsdl = self.imgui_win.as_mut().unwrap();
		imsdl.handle_event(&mut imctx, &event);
		imsdl.ignore_event(&event)
	}
}

impl Editor {
	pub fn add_element<T>(&mut self, element: &Rc<RefCell<T>>)
	where
		T: ui::Element + 'static,
	{
		let element_strong: Rc<RefCell<dyn ui::Element>> = element.clone();
		self.ui_elements.push(Rc::downgrade(&element_strong));
	}

	pub fn render_frame(&mut self, engine: &mut engine::Engine) -> engine::utility::Result<()> {
		{
			let event_pump = self.display().borrow().event_pump()?;
			let window = self.sdl_window.as_mut().unwrap();
			let imsdl = self.imgui_win.as_mut().unwrap();
			let imctx = self.imgui_ctx.as_mut().unwrap();
			imsdl.prepare_frame(imctx.io_mut(), &window, &event_pump.mouse_state());
		}

		{
			let now = Instant::now();
			let delta = now - self.last_frame;
			let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
			self.last_frame = now;

			let imctx = self.imgui_ctx.as_mut().unwrap();
			imctx.io_mut().delta_time = delta_s;
		}

		let ui_builder = self.imgui_ctx.as_mut().unwrap().frame();
		self.ui_elements
			.retain(|element| element.strong_count() > 0);
		for element in self.ui_elements.iter() {
			element.upgrade().unwrap().borrow_mut().render(
				engine,
				&self.asset_manager,
				&ui_builder,
			);
		}

		unsafe {
			gl::ClearColor(0.2, 0.2, 0.2, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		{
			let window = self.sdl_window.as_mut().unwrap();
			let imsdl = self.imgui_win.as_mut().unwrap();
			let imren = self.imgui_renderer.as_mut().unwrap();
			imsdl.prepare_render(&ui_builder, &window);
			imren.render(ui_builder);

			window.gl_swap_window();
		}

		Ok(())
	}
}
