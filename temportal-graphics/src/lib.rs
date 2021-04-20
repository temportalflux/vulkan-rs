extern crate sdl2;
extern crate vk_mem;

/// Various forwarded/exposed structures from Vulkan/Erupt
pub mod structs {
	pub use erupt::vk::ComponentMapping;
	pub use erupt::vk::Extent2D;
	pub use erupt::vk::ImageSubresourceRange;
	pub use erupt::vk::Offset2D;
	pub use erupt::vk::Rect2D;
}

/// Various forwarded/exposed enumerations from Vulkan/Erupt
#[path = "flags/_.rs"]
pub mod flags;

#[path = "context.rs"]
mod context;
pub use context::Context;

#[path = "general/lib.rs"]
mod general;
pub use general::*;

#[path = "object/lib.rs"]
mod object;
pub use object::*;

/// Instruction related structs.
#[path = "command/lib.rs"]
pub mod command;

/// Vulkan Instance related structs.
#[path = "instance/lib.rs"]
pub mod instance;

/// Physical (GPU) and Logical device related structs.
#[path = "device/lib.rs"]
pub mod device;

/// General-use traits and macros.
#[path = "utility/lib.rs"]
pub mod utility;

/// Structs used in the creation or representation of Pipelines and Pipeline Layouts.
#[path = "pipeline/lib.rs"]
pub mod pipeline;

/// Structs used in the creation or representation of a Render Pass.
#[path = "renderpass/lib.rs"]
pub mod renderpass;

pub mod shader;
