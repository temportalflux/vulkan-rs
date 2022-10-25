use crate::{backend, device::logical, sampler::Builder, utility};
use std::sync;

pub struct Sampler {
	internal: backend::vk::Sampler,
	device: sync::Arc<logical::Device>,
}

impl Sampler {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::Sampler,
	) -> Sampler {
		Sampler { device, internal }
	}
}

impl std::ops::Deref for Sampler {
	type Target = backend::vk::Sampler;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Sampler {
	fn drop(&mut self) {
		unsafe { self.device.destroy_sampler(self.internal, None) };
	}
}

impl utility::HandledObject for Sampler {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Sampler as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
