extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;
extern crate shaderc;

pub use temportal_engine as engine;

#[path = "asset/_.rs"]
pub mod asset;

mod editor;
pub use editor::*;

#[path = "ui/_.rs"]
pub mod ui;
