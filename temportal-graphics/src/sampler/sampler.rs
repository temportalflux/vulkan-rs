use crate::{backend, device::logical, sampler::Builder, utility::VulkanObject};
use std::rc::Rc;

pub struct Sampler {
	internal: backend::vk::Sampler,
	device: Rc<logical::Device>,
}

impl Sampler {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub fn from(device: Rc<logical::Device>, internal: backend::vk::Sampler) -> Sampler {
		Sampler { device, internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Buffer`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::Sampler> for Sampler {
	fn unwrap(&self) -> &backend::vk::Sampler {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Sampler {
		&mut self.internal
	}
}

impl Drop for Sampler {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.unwrap().destroy_sampler(self.internal, None) };
	}
}
