use crate::backend;

/// A user-owned singleton which holds data about allocators and api-level availability.
pub struct Context {
	pub loader: backend::Entry,
	pub valid_instance_extensions: Vec<String>,
	pub valid_instance_layers: Vec<String>,
}

impl Context {
	pub fn new() -> anyhow::Result<Context> {
		let loader = unsafe { backend::Entry::load() }?;
		let valid_instance_extensions = Context::get_instance_extensions(&loader);
		let valid_instance_layers = Context::get_instance_layers(&loader);
		Ok(Context {
			loader,
			valid_instance_extensions,
			valid_instance_layers,
		})
	}

	fn get_instance_extensions(loader: &backend::Entry) -> Vec<String> {
		let mut valid_instance_extensions: Vec<String> = Vec::new();
		unsafe {
			let ext_props = loader
				.enumerate_instance_extension_properties(None)
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
		valid_instance_extensions
	}

	fn get_instance_layers(loader: &backend::Entry) -> Vec<String> {
		let mut valid_instance_layers: Vec<String> = Vec::new();
		unsafe {
			let layer_props = loader.enumerate_instance_layer_properties().unwrap();
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
		valid_instance_layers
	}

	///! Returns true if the provided layer name is in the list of valid layers for the vulkan instance.
	pub fn is_valid_instance_layer(&self, name: &String) -> bool {
		self.valid_instance_layers.contains(&name)
	}
}
