#[path = "attachment.rs"]
mod attachment;
pub use attachment::*;

#[path = "clear_value.rs"]
mod clear_value;
pub use clear_value::*;

#[path = "info.rs"]
mod info;
pub use info::*;

#[path = "instruction.rs"]
mod instruction;
pub use instruction::*;

#[path = "subpass.rs"]
mod subpass;
pub use subpass::*;

#[path = "pass.rs"]
mod pass;
pub use pass::*;
