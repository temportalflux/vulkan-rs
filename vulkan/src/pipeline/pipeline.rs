use crate::{backend, device::logical, pipeline::Builder, utility};

use std::sync;

/// A vulkan Pipeline. A given pipeline is only valid for a specific [`Render Pass`](crate::renderpass::Pass),
/// and is used to issue commands to the [`GPU`](crate::device::physical::Device).
pub struct Pipeline {
	internal: backend::vk::Pipeline,
	device: sync::Arc<logical::Device>,
	name: String,
}

impl Pipeline {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::Pipeline,
		name: String,
	) -> Pipeline {
		Pipeline {
			device,
			internal,
			name,
		}
	}
}

impl std::ops::Deref for Pipeline {
	type Target = backend::vk::Pipeline;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Pipeline {
	fn drop(&mut self) {
		log::debug!(target: crate::LOG, "Dropping Pipeline: {:?}", self.name);
		unsafe { self.device.destroy_pipeline(self.internal, None) };
	}
}

impl utility::HandledObject for Pipeline {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Pipeline as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
