use crate::{backend, device::logical, utility};

use std::sync;

/// The [`Render Pass`](backend::vk::RenderPass) used by Vulkan
/// to define when pipeline instructions can be issued
/// and what attachments are used.
pub struct Pass {
	subpass_order: Vec<String>,
	internal: backend::vk::RenderPass,
	device: sync::Arc<logical::Device>,
	name: String,
}

impl Pass {
	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::RenderPass,
		subpass_order: Vec<String>,
		name: String,
	) -> Pass {
		Pass {
			device,
			internal,
			subpass_order,
			name,
		}
	}

	pub fn subpass_index(&self, subpass_id: &Option<String>) -> u32 {
		match subpass_id {
			Some(subpass_id) => self
				.subpass_order
				.iter()
				.position(|id| *id == *subpass_id)
				.unwrap_or(0) as u32,
			None => 0,
		}
	}
}

impl std::ops::Deref for Pass {
	type Target = backend::vk::RenderPass;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Pass {
	fn drop(&mut self) {
		log::debug!(target: crate::LOG, "Dropping RenderPass: {:?}", self.name);
		unsafe { self.device.destroy_render_pass(self.internal, None) };
	}
}

impl utility::HandledObject for Pass {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::RenderPass as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
