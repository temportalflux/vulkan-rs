use crate::{backend, device::logical, image, image_view::Builder};

use std::sync;

/// A wrapper around [`Image View`](backend::vk::ImageView).
pub struct View {
	internal: backend::vk::ImageView,
	image: sync::Arc<image::Image>,
	device: sync::Arc<logical::Device>,
}

impl View {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub fn from(
		device: sync::Arc<logical::Device>,
		image: sync::Arc<image::Image>,
		internal: backend::vk::ImageView,
	) -> View {
		View {
			device,
			image,
			internal,
		}
	}

	pub fn image(&self) -> &sync::Arc<image::Image> {
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
