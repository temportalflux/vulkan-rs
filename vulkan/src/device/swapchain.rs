use crate::{
	command::{SyncObject}, device::logical, flags, image::Image, image_view::View, structs, utility,
};
use std::sync::Arc;

#[path = "swapchain/khr.rs"]
pub mod khr;
#[path = "swapchain/memory.rs"]
pub mod memory;

/// An image returned by [`acquire_next_image`](Swapchain::acquire_next_image).
pub enum AcquiredImage {
	/// The next image to render to has been determined.
	Available(usize),
	/// The next image to render to has been determined, but the swapchain is
	/// outdated and should be recreated with an updated resolution/extent.
	Suboptimal(usize),
}

/// Generic trait implemented by a builder which creates a particular implementation of [`Swapchain`].
pub trait SwapchainBuilder {
	/// Set the extent/resolution of the swapchain.
	fn set_image_extent(&mut self, resolution: structs::Extent2D);
	
	/// Returns the extent/resolution of the images the swapchain creates.
	fn image_extent(&self) -> &structs::Extent2D;
	
	/// Returns the number of images the swapchain is configured to create.
	fn image_count(&self) -> usize;

	/// Returns the format of the images the swapchain creates.
	fn image_format(&self) -> flags::format::Format;
	
	/// Change the surface transform of the swapchain to be built.
	fn set_surface_transform(&mut self, transform: flags::SurfaceTransform);

	/// Change the presentation mode of the swapchain to be built.
	fn set_present_mode(&mut self, mode: flags::PresentMode);

	/// Build the swapchain based on current configuration data.
	/// This is a non-consuming operation, so that swapchains can be rebuilt from the same builder.
	fn build(
		&self,
		old: Option<Box<dyn Swapchain + 'static + Send + Sync>>,
	) -> anyhow::Result<Box<dyn Swapchain + 'static + Send + Sync>>;
}

/// A generic API for interacting with different kinds of swapchains.
pub trait Swapchain {

	/// Returns the logical device that the swapchain was built using.
	fn device(&self) -> &Arc<logical::Device>;

	/// Downcasts the swapchain to a KHR swapchain so it can be used internally.
	fn as_khr(&self) -> Option<&khr::Swapchain> {
		None
	}

	/// Returns the number of images the swapchain is configured to create.
	fn image_count(&self) -> usize;

	/// Returns the extent/resolution of the images the swapchain creates.
	fn image_extent(&self) -> &structs::Extent2D;

	/// Creates a number of images equivalent to [`image_count`](Swapchain::image_count).
	fn create_images(&self) -> anyhow::Result<Vec<Arc<Image>>>;

	/// Creates a number of image views equivalent to [`image_count`](Swapchain::image_count),
	/// delegating to [`create_images`](Swapchain::create_images) to create the actual images which are wrapped in image views.
	fn create_image_views(&self) -> anyhow::Result<Vec<Arc<View>>> {
		use utility::{BuildFromDevice, NameableBuilder, NamedObject};
		let mut views = Vec::new();
		for image in self.create_images()?.into_iter() {
			views.push(Arc::new(
				View::builder()
					.with_optname(image.name().as_ref().map(|name| format!("{}.View", name)))
					.for_image(image)
					.with_view_type(flags::ImageViewType::TYPE_2D)
					.with_range(
						structs::subresource::Range::default()
							.with_aspect(flags::ImageAspect::COLOR),
					)
					.build(&self.device())?,
			));
		}
		Ok(views)
	}

	/// Attempts to determine what the next available image to submit/present to would be.
	fn acquire_next_image(
		&self,
		timeout: u64,
		barrier: SyncObject,
	) -> anyhow::Result<AcquiredImage>;
}
