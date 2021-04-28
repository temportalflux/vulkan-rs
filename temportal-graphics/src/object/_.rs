/// Image-related structs (for storing, editting, and viewing textures or render-pass results on the GPU).
#[path = "image/_.rs"]
pub mod image;

/// Buffer-relevant structs (for sending/storing chunks of data on the GPU).
#[path = "buffer/_.rs"]
pub mod buffer;

mod allocation_info;
pub use allocation_info::*;
