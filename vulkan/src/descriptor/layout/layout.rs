use crate::{backend, descriptor::layout::Builder, device::logical, utility};
use std::sync;

/// Defines the format that descriptor sets are created in.
pub struct SetLayout {
	internal: backend::vk::DescriptorSetLayout,
	device: sync::Arc<logical::Device>,
	name: String,
}

impl SetLayout {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::DescriptorSetLayout,
		name: String,
	) -> SetLayout {
		SetLayout {
			device,
			internal,
			name,
		}
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
		log::debug!(
			target: crate::LOG,
			"Dropping DescriptorSetLayout: {:?}",
			self.name
		);
		unsafe {
			self.device
				.destroy_descriptor_set_layout(self.internal, None);
		}
	}
}

impl utility::HandledObject for SetLayout {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::DescriptorSetLayout as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
