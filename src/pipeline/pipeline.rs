use crate::{backend, device::logical};

use std::sync;

/// A vulkan Pipeline. A given pipeline is only valid for a specific [`Render Pass`](crate::renderpass::Pass),
/// and is used to issue commands to the [`GPU`](crate::device::physical::Device).
pub struct Pipeline {
	internal: backend::vk::Pipeline,
	device: sync::Arc<logical::Device>,
}

impl Pipeline {
	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::Pipeline,
	) -> Pipeline {
		Pipeline { device, internal }
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
		use backend::version::DeviceV1_0;
		unsafe { self.device.destroy_pipeline(self.internal, None) };
	}
}
