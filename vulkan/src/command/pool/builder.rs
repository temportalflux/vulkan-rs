use super::Pool;
use crate::{backend, device::logical, flags, utility};
use std::sync;

pub struct Builder {
	name: Option<String>,
	queue_family_index: usize,
	flags: Option<flags::CommandPoolCreate>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			name: None,
			queue_family_index: 0,
			flags: None,
		}
	}
}

impl Builder {
	pub fn with_queue_family_index(mut self, idx: usize) -> Self {
		self.queue_family_index = idx;
		self
	}

	pub fn with_flag(mut self, flag: flags::CommandPoolCreate) -> Self {
		self.flags = Some(flag);
		self
	}
}

impl utility::NameableBuilder for Builder {
	fn set_optname(&mut self, name: Option<String>) {
		self.name = name;
	}

	fn name(&self) -> &Option<String> {
		&self.name
	}
}

impl utility::BuildFromDevice for Builder {
	type Output = Pool;
	/// Creates a command pool from a device, queue, and a flag indicating the kind of command pool it is.
	fn build(self, device: &sync::Arc<logical::Device>) -> utility::Result<Self::Output> {
		use backend::version::DeviceV1_0;
		use utility::NameableBuilder;
		let info = backend::vk::CommandPoolCreateInfo::builder()
			.queue_family_index(self.queue_family_index as u32)
			.flags(self.flags.unwrap_or_default())
			.build();
		let internal = unsafe { device.create_command_pool(&info, None) }?;
		let pool = Pool::from(device.clone(), self.name().clone(), internal);
		self.set_object_name(device, &pool);
		Ok(pool)
	}
}
