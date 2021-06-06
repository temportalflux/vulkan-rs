//! # Vulkan-rs
//!
//! A rust-safe interface for handling Vulkan structures and sending commands to / rendering on the GPU.
//! Uses [`ash`] and [`vk_mem`] under the hood to handle the actual interfacing with the
//! [C/C++ Vulkan Headers Library](https://github.com/KhronosGroup/Vulkan-Headers).

extern crate memoffset;
extern crate vk_mem;

pub use ash as backend;

/// The log category for the graphics library.
pub static LOG: &'static str = "vulkan";

/// Applies the [`Object`](`pipeline::state::vertex::Object`) trait to some struct.
#[cfg(feature = "derive")]
pub use vulkan_rs_derive::vertex_object;

/// Allocation management structures for handling the creation graphics objects
/// with memory on the CPU and/or GPU (like [`buffers`](buffer::Buffer) and [`images`](image::Image)).
#[path = "alloc/_.rs"]
pub mod alloc;

/// Buffer-relevant structs (for sending/storing chunks of data on the GPU).
#[path = "buffer/_.rs"]
pub mod buffer;

#[path = "context.rs"]
mod context;
pub use context::Context;

/// Structures related to submitting command instructions to the GPU.
#[path = "command/_.rs"]
pub mod command;

/// Structures related to describing data that gets sent to the GPU via buffers and pipelines.
#[path = "descriptor/_.rs"]
pub mod descriptor;

/// Physical (GPU) and Logical device related structs.
#[path = "device/_.rs"]
pub mod device;

/// Various forwarded/exposed enumerations from Vulkan/Backend
#[path = "flags/_.rs"]
pub mod flags;

#[path = "general/_.rs"]
mod general;
pub use general::*;

/// Vulkan Instance related structs.
#[path = "instance/_.rs"]
pub mod instance;

/// Image-related structs (for storing, editting, and viewing textures or render-pass results on the GPU).
#[path = "image/_.rs"]
pub mod image;

/// Structs for creating views for [`images`](image::Image).
#[path = "image_view/_.rs"]
pub mod image_view;

/// Structs used in the creation or representation of Pipelines and Pipeline Layouts.
#[path = "pipeline/_.rs"]
pub mod pipeline;

/// Structs used in the creation or representation of a Render Pass.
#[path = "renderpass/_.rs"]
pub mod renderpass;

/// Structs used for creating graphics samplers, which tell the GPU how to read an [`image`](image::Image).
#[path = "sampler/_.rs"]
pub mod sampler;

/// Structs relating to how the GPU runs calculations, often used for describing how to determine what the color of a pixel is.
pub mod shader;

/// Various forwarded/exposed structures from Vulkan/Backend
#[path = "structs/_.rs"]
pub mod structs;

/// General-use traits and macros.
#[path = "utility/_.rs"]
pub mod utility;
