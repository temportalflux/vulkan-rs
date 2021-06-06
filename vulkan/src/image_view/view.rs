use crate::{backend, device::logical, image::Image, image_view::Builder};

use std::sync;

/// A vulkan object used to own and view an [`Image`].
/// When a `View` object is dropped, the underlying vulkan ImageView is also dropped.
/// A view will own its [`Image`], so even if the user drops the image pointer,
/// the view will keep the image around until the view is dropped.
pub struct View {
	internal: backend::vk::ImageView,
	image: sync::Arc<Image>,
	device: sync::Arc<logical::Device>,
}

impl View {
	/// Helper method for creating a default buffer builder.
	pub fn builder() -> Builder {
		Builder::default()
	}

	/// Constructs the buffer object from a completed [`Builder`].
	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		image: sync::Arc<Image>,
		internal: backend::vk::ImageView,
	) -> View {
		View {
			device,
			image,
			internal,
		}
	}

	/// Returns the image that the view owns/is connected to.
	pub fn image(&self) -> &sync::Arc<Image> {
		&self.image
	}
}

impl std::ops::Deref for View {
	type Target = backend::vk::ImageView;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for View {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.destroy_image_view(self.internal, None) };
	}
}
