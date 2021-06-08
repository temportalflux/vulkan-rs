use crate::{backend, descriptor::layout::Builder, device::logical};
use std::sync;

/// Defines the format that descriptor sets are created in.
pub struct SetLayout {
	internal: backend::vk::DescriptorSetLayout,
	device: sync::Arc<logical::Device>,
}

impl SetLayout {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::DescriptorSetLayout,
	) -> SetLayout {
		SetLayout { device, internal }
	}
}

impl std::ops::Deref for SetLayout {
	type Target = backend::vk::DescriptorSetLayout;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for SetLayout {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.destroy_descriptor_set_layout(self.internal, None);
		}
	}
}
