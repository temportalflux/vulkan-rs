extern crate sdl2;

use std::error::Error;
use structopt::StructOpt;

mod instance;
pub use instance::*;

#[derive(Debug, StructOpt)]
struct Opt {
	/// Use validation layers
	#[structopt(short, long)]
	validation_layers: bool,
}

#[macro_export]
macro_rules! version {
	($major:expr, $minor:expr, $patch:expr) => {
		temportal_graphics::AppInfo::make_version($major, $minor, $patch)
	};
}

pub fn create_instance(
	app_info: &AppInfo,
	window_extensions: Vec<&'static str>,
) -> Result<Instance, Box<dyn Error>> {
	let mut instance_info = InstanceInfo::new().app_info(app_info.clone());
	for name_slice in window_extensions.iter() {
		instance_info.add_extension(name_slice);
	}

	let opt = Opt::from_args();
	if opt.validation_layers {
		instance_info.add_extension("VK_EXT_debug_utils");
		instance_info.add_layer("VK_LAYER_KHRONOS_validation");
	}

	Instance::new(&mut instance_info)
}
