/// Physical device related structs (for know about hardware).
pub mod physical;

/// Logical device related structs (for creating things that send instructions to hardware).
#[path = "logical/_.rs"]
pub mod logical;

/// Swapchain related structs (for being able to swap different iamges out for frames).
#[path = "swapchain/_.rs"]
pub mod swapchain;
