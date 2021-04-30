mod lib;
use lib::*;
fn main() -> engine::utility::VoidResult {
	#[cfg(feature = "profile")]
	{
		optick::start_capture();
	}
	lib::run()?;
	#[cfg(feature = "profile")]
	{
		optick::stop_capture(name());
	}
	Ok(())
}
