use crate::utility::VulkanObject;
use erupt;

/// The wrapper for the [`Vulkan Surface`](erupt::vk::SurfaceKHR).
/// This represents the canvas/display within the provided [`window`](raw_window_handle::HasRawWindowHandle)
/// that Vulkan should draw to. (see [`create_surface`](crate::instance::Instance::create_surface))
pub struct Surface {
	_internal: erupt::vk::SurfaceKHR,
}

impl Surface {
	/// The internal constructor. Users should use [`create_surface`](crate::instance::Instance::create_surface) to create a surface.
	pub fn from(_internal: erupt::vk::SurfaceKHR) -> Surface {
		Surface { _internal }
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::SurfaceKHR`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::SurfaceKHR> for Surface {
	fn unwrap(&self) -> &erupt::vk::SurfaceKHR {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::SurfaceKHR {
		&mut self._internal
	}
}
