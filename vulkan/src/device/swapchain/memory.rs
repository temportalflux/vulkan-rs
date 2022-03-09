//! A swapchain used to present to some in-memory render target.

#[path = "memory/builder.rs"]
mod builder;
pub use builder::*;

#[path = "memory/swapchain.rs"]
mod swapchain;
pub use swapchain::*;
