use crate::{backend, instance, utility::VulkanObject};
use std::sync;

/// The wrapper for the [`Vulkan Surface`](backend::vk::SurfaceKHR).
/// This represents the canvas/display within the provided [`window`](raw_window_handle::HasRawWindowHandle)
/// that Vulkan should draw to. (see [`create_surface`](crate::instance::Instance::create_surface))
pub struct Surface {
	_internal: backend::vk::SurfaceKHR,
	_instance: sync::Arc<instance::Instance>,
}

impl Surface {
	/// The internal constructor. Users should use [`create_surface`](crate::instance::Instance::create_surface) to create a surface.
	pub fn from(
		_instance: sync::Arc<instance::Instance>,
		_internal: backend::vk::SurfaceKHR,
	) -> Surface {
		Surface {
			_instance,
			_internal,
		}
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::SurfaceKHR`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::SurfaceKHR> for Surface {
	fn unwrap(&self) -> &backend::vk::SurfaceKHR {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::SurfaceKHR {
		&mut self._internal
	}
}

impl Drop for Surface {
	fn drop(&mut self) {
		self._instance.destroy_surface(self._internal);
	}
}
