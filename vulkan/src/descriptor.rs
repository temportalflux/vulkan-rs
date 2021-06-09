//! Structures related to describing data that gets sent to the GPU via buffers and pipelines.
//!
//! # Overview
//!
//! The GPU requires a description of how [`buffers`](crate::buffer::Buffer)
//! and [`images`](crate::image::Image) are mapped to bindings that the shader can read.
//!
//! These bindings are similar to [`attributes`](crate::pipeline::state::vertex::Attribute),
//! in that both indicate to the GPU how to read data from buffers populated by the CPU.
//! Descriptors differ from attributes, however, because attributes describe how
//! vertex buffers (and by extension, instance buffers) format groups of data,
//! whereas descriptors and desc. sets are read per draw call, not per vertex.
//!
//! The format of a singular descriptor in a glsl shader may look like:
//! - `layout(set = #, binding = #) uniform mat4 camera_proj;`
//! - `layout(set = #, binding = #) uniform sampler2D some_texture;`
//!
//! And a descriptor set would be a group of bindings:
//! ```
//! layout(set = 0, binding = 0) uniform sampler2D tex_main;
//! layout(set = 0, binding = 1) uniform sampler2D tex_normal_map;
//! layout(set = 0, binding = 2) uniform sampler2D tex_metallic;
//! layout(set = 0, binding = 3) uniform sampler2D tex_shiny;
//! ```
//!
//! See [glsl layout qualifer docs](https://www.khronos.org/opengl/wiki/Layout_Qualifier_%28GLSL%29#Binding_points)
//! for more info on glsl layout bindings.
//!
//! Individual descriptors cannot be created, you must use a descriptor layout and create a set based on a layout
//! in order to tell the GPU what bindings point to what buffers/images. There is nothing preventing you
//! from creating a layout and set which only contain 1 descriptor though!
//! (This is often done for very simple shaders for sending a texture or a camera uniform block.)
//!

pub mod layout;
pub mod pool;
mod set;
pub use set::*;
pub mod update;
