use crate::{engine, ui, Editor};
use std::{
	cell::RefCell,
	rc::{Rc, Weak},
	time::Instant,
};

pub struct Ui {
	ui_elements: Vec<Weak<RefCell<dyn ui::Element>>>,
	last_frame: Instant,
	imgui_renderer: imgui_opengl_renderer::Renderer,
	imgui_win: imgui_sdl2::ImguiSdl2,
	imgui_ctx: imgui::Context,
	_gl_context: sdl2::video::GLContext,
	sdl_window: sdl2::video::Window,
}

impl Ui {
	pub fn new(
		display: &engine::display::Manager,
		title: &str,
		width: u32,
		height: u32,
	) -> engine::utility::Result<Ui> {
		let video = display.video_subsystem()?;
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		gl_attr.set_context_version(3, 0);

		let sdl_window = engine::utility::as_window_error(
			video
				.window(title, width, height)
				.position_centered()
				.resizable()
				.opengl()
				.allow_highdpi()
				.build(),
		)?;

		let _gl_context = sdl_window
			.gl_create_context()
			.expect("Couldn't create GL context");
		gl::load_with(|s| video.gl_get_proc_address(s) as _);

		let mut imgui_ctx = imgui::Context::create();
		imgui_ctx.set_ini_filename(None);

		let imgui_win = imgui_sdl2::ImguiSdl2::new(&mut imgui_ctx, &sdl_window);

		let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_ctx, |s| {
			video.gl_get_proc_address(s) as _
		});

		Ok(Ui {
			sdl_window,
			_gl_context,
			imgui_ctx,
			imgui_win,
			imgui_renderer,
			last_frame: Instant::now(),
			ui_elements: Vec::new(),
		})
	}
}

impl engine::display::EventListener for Ui {
	fn on_event(&mut self, event: &sdl2::event::Event) -> bool {
		self.imgui_win.handle_event(&mut self.imgui_ctx, &event);
		self.imgui_win.ignore_event(&event)
	}
}

impl Ui {
	pub fn add_element<T>(&mut self, element: &Rc<RefCell<T>>)
	where
		T: ui::Element + 'static,
	{
		let element_strong: Rc<RefCell<dyn ui::Element>> = element.clone();
		self.ui_elements.push(Rc::downgrade(&element_strong));
	}

	pub fn render_frame(
		&mut self,
		editor: &mut Editor,
		event_pump: sdl2::EventPump,
	) -> engine::utility::VoidResult {
		optick::next_frame();
		self.imgui_win.prepare_frame(
			self.imgui_ctx.io_mut(),
			&self.sdl_window,
			&event_pump.mouse_state(),
		);

		let now = Instant::now();
		let delta = now - self.last_frame;
		let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
		self.last_frame = now;
		self.imgui_ctx.io_mut().delta_time = delta_s;

		self.ui_elements
			.retain(|element| element.strong_count() > 0);
		let ui_builder = self.imgui_ctx.frame();
		for element in self.ui_elements.iter() {
			element
				.upgrade()
				.unwrap()
				.borrow_mut()
				.render(editor, &ui_builder);
		}

		unsafe {
			gl::ClearColor(0.2, 0.2, 0.2, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		self.imgui_win.prepare_render(&ui_builder, &self.sdl_window);
		self.imgui_renderer.render(ui_builder);

		self.sdl_window.gl_swap_window();

		Ok(())
	}
}
