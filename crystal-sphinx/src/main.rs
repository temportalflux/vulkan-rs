mod lib;
use lib::*;
fn main() -> engine::utility::VoidResult {
	lib::run(std::env!("CARGO_PKG_NAME"))
}
