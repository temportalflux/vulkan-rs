extern crate sdl2;
extern crate vk_mem;

use erupt;

#[path = "context.rs"]
mod context;
#[path = "device/lib.rs"]
pub mod device;
#[path = "general/lib.rs"]
pub mod general;
#[path = "instance/lib.rs"]
pub mod instance;
#[path = "utility/lib.rs"]
pub mod utility;

pub use context::Context;
pub use erupt::vk::ColorSpaceKHR as ColorSpace;
pub use erupt::vk::Format;
pub use erupt::vk::PresentModeKHR as PresentMode;
pub use erupt::vk::QueueFlags;
pub use general::AppInfo;
