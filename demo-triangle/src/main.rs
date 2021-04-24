mod lib;
pub use lib::*;
fn main() -> Result<(), engine::utility::AnyError> {
	lib::run(std::env!("CARGO_PKG_NAME"))
}
