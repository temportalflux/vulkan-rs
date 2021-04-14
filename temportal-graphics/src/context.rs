use erupt;
use std::error::Error;

pub struct Context {
	pub loader: erupt::DefaultEntryLoader,
	pub valid_instance_extensions: Vec<String>,
	pub valid_instance_layers: Vec<String>,
}

impl Context {
	pub fn new() -> Result<Context, Box<dyn Error>> {
		let loader = erupt::EntryLoader::new()?;

		let mut valid_instance_extensions: Vec<String> = Vec::new();
		unsafe {
			let ext_props = loader
				.enumerate_instance_extension_properties(None, None)
				.unwrap();
			for prop in ext_props {
				// Convert the os-level string to a rust string
				valid_instance_extensions.push(
					std::ffi::CStr::from_ptr(&prop.extension_name as *const i8)
						.to_owned()
						.into_string()
						.unwrap(),
				);
			}
		}

		let mut valid_instance_layers: Vec<String> = Vec::new();
		unsafe {
			let layer_props = loader.enumerate_instance_layer_properties(None).unwrap();
			for prop in layer_props {
				// Convert the os-level string to a rust string
				valid_instance_layers.push(
					std::ffi::CStr::from_ptr(&prop.layer_name as *const i8)
						.to_owned()
						.into_string()
						.unwrap(),
				);
			}
		}
		Ok(Context {
			loader,
			valid_instance_extensions,
			valid_instance_layers,
		})
	}

	pub fn is_valid_instance_layer(&self, name: &String) -> bool {
		self.valid_instance_layers.contains(&name)
	}
}
