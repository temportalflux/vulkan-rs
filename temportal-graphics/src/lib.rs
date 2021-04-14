extern crate sdl2;
extern crate vk_mem;

use erupt;
use erupt::utils::surface::enumerate_required_extensions;
use std::error::Error;
use structopt::StructOpt;

mod context;
pub use context::*;
mod instance;
pub use instance::*;
mod physical_device;
pub use erupt::vk::ColorSpaceKHR as ColorSpace;
pub use erupt::vk::Format;
pub use erupt::vk::PhysicalDeviceType as PhysicalDeviceKind;
pub use erupt::vk::PresentModeKHR as PresentMode;
pub use erupt::vk::QueueFlags;
pub use physical_device::*;

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
	ctx: &Context,
	app_info: &AppInfo,
	window_handle: &impl raw_window_handle::HasRawWindowHandle,
) -> Result<Instance, Box<dyn Error>> {
	let mut instance_info = InstanceInfo::new().app_info(app_info.clone());

	let window_extensions = enumerate_required_extensions(window_handle).unwrap();
	instance_info.append_raw_extensions(window_extensions);

	let opt = Opt::from_args();
	if opt.validation_layers {
		instance_info.add_extension("VK_EXT_debug_utils");
		instance_info.add_layer("VK_LAYER_KHRONOS_validation");
	}

	Instance::new(&ctx, &mut instance_info, opt.validation_layers)
}
