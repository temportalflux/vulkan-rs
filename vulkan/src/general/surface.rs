use crate::{backend, instance};
use std::sync;

/// The wrapper for the [`Vulkan Surface`](backend::vk::SurfaceKHR).
/// This represents the canvas/display within the provided [`window`](raw_window_handle::HasRawWindowHandle)
/// that Vulkan should draw to. (see [`create_surface`](crate::instance::Instance::create_surface))
pub struct Surface {
	internal: backend::vk::SurfaceKHR,
	instance: sync::Arc<instance::Instance>,
}

impl Surface {
	/// The internal constructor. Users should use [`create_surface`](crate::instance::Instance::create_surface) to create a surface.
	pub(crate) fn from(
		instance: sync::Arc<instance::Instance>,
		internal: backend::vk::SurfaceKHR,
	) -> Surface {
		Surface { instance, internal }
	}
}

impl std::ops::Deref for Surface {
	type Target = backend::vk::SurfaceKHR;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Surface {
	fn drop(&mut self) {
		self.instance.destroy_surface(self.internal);
	}
}
