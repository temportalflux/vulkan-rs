extern crate sdl2;
extern crate vk_mem;

pub use erupt::vk::ColorSpaceKHR as ColorSpace;
pub use erupt::vk::CompositeAlphaFlagBitsKHR as CompositeAlpha;
pub use erupt::vk::Extent2D;
pub use erupt::vk::Format;
pub use erupt::vk::ImageUsageFlags;
pub use erupt::vk::PresentModeKHR as PresentMode;
pub use erupt::vk::QueueFlags;
pub use erupt::vk::SharingMode;
pub use erupt::vk::SurfaceTransformFlagBitsKHR as SurfaceTransform;

#[path = "context.rs"]
mod context;
pub use context::Context;

#[path = "general/lib.rs"]
mod general;
pub use general::*;

#[path = "object/lib.rs"]
mod object;
pub use object::*;

/// Vulkan Instance related structs.
#[path = "instance/lib.rs"]
pub mod instance;

/// Physical (GPU) and Logical device related structs.
#[path = "device/lib.rs"]
pub mod device;

/// General-use traits and macros.
#[path = "utility/lib.rs"]
pub mod utility;
