/// Physical device related structs (for know about hardware).
#[path = "physical.rs"]
pub mod physical;

/// Logical device related structs (for creating things that send instructions to hardware).
#[path = "logical/lib.rs"]
pub mod logical;

/// Swapchain related structs (for being able to swap different iamges out for frames).
#[path = "swapchain/lib.rs"]
pub mod swapchain;
