extern crate sdl2;
extern crate vk_mem;

pub use ash as backend;

/// Various forwarded/exposed structures from Vulkan/Backend
pub mod structs {
	pub use crate::backend::vk::ComponentMapping;
	pub use crate::backend::vk::Extent2D;
	pub use crate::backend::vk::Extent3D;
	pub use crate::backend::vk::ImageSubresourceRange;
	pub use crate::backend::vk::Offset2D;
	pub use crate::backend::vk::Rect2D;
}

pub static LOG: &'static str = "graphics";

#[path = "alloc/_.rs"]
pub mod alloc;

/// Various forwarded/exposed enumerations from Vulkan/Backend
#[path = "flags/_.rs"]
pub mod flags;

#[path = "context.rs"]
mod context;
pub use context::Context;

#[path = "descriptor/_.rs"]
pub mod descriptor;

#[path = "general/_.rs"]
mod general;
pub use general::*;

#[path = "object/_.rs"]
mod object;
pub use object::*;

/// Instruction related structs.
#[path = "command/_.rs"]
pub mod command;

/// Vulkan Instance related structs.
#[path = "instance/_.rs"]
pub mod instance;

/// Physical (GPU) and Logical device related structs.
#[path = "device/_.rs"]
pub mod device;

/// General-use traits and macros.
#[path = "utility/_.rs"]
pub mod utility;

/// Structs used in the creation or representation of Pipelines and Pipeline Layouts.
#[path = "pipeline/_.rs"]
pub mod pipeline;

/// Structs used in the creation or representation of a Render Pass.
#[path = "renderpass/_.rs"]
pub mod renderpass;

pub mod shader;
