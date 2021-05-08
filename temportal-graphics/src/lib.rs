extern crate memoffset;
extern crate vk_mem;

pub use ash as backend;

pub static LOG: &'static str = "graphics";

#[path = "alloc/_.rs"]
pub mod alloc;

/// Buffer-relevant structs (for sending/storing chunks of data on the GPU).
#[path = "buffer/_.rs"]
pub mod buffer;

#[path = "context.rs"]
mod context;
pub use context::Context;

/// Instruction related structs.
#[path = "command/_.rs"]
pub mod command;

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

#[path = "image_view/_.rs"]
pub mod image_view;

/// Structs used in the creation or representation of Pipelines and Pipeline Layouts.
#[path = "pipeline/_.rs"]
pub mod pipeline;

/// Structs used in the creation or representation of a Render Pass.
#[path = "renderpass/_.rs"]
pub mod renderpass;

#[path = "sampler/_.rs"]
pub mod sampler;

pub mod shader;

/// Various forwarded/exposed structures from Vulkan/Backend
#[path = "structs/_.rs"]
pub mod structs;

/// General-use traits and macros.
#[path = "utility/_.rs"]
pub mod utility;
