use crate::{
	backend, command, device::logical, flags, image::Image, image_view::View, structs, utility,
};
use std::sync::Arc;

#[path = "swapchain/khr.rs"]
pub mod khr;
#[path = "swapchain/memory.rs"]
pub mod memory;

pub enum ImageAcquisitionBarrier<'a> {
	Semaphore(&'a command::Semaphore),
	Fence(&'a command::Fence),
}

pub enum AcquiredImage {
	Available(usize),
	Suboptimal(usize),
}

pub trait SwapchainBuilder {
	fn image_count(&self) -> usize;
	fn image_format(&self) -> flags::format::Format;
	fn set_image_extent(&mut self, resolution: structs::Extent2D);
	fn set_surface_transform(&mut self, transform: flags::SurfaceTransform);
	fn set_present_mode(&mut self, mode: flags::PresentMode);
	fn build(
		&self,
		old: Option<Box<dyn Swapchain + 'static>>,
	) -> anyhow::Result<Box<dyn Swapchain + 'static>>;
}

pub trait Swapchain {
	fn device(&self) -> &Arc<logical::Device>;

	fn resolve_to_khr(&self) -> Option<backend::vk::SwapchainKHR> {
		None
	}

	fn image_count(&self) -> usize;
	fn image_extent(&self) -> &structs::Extent2D;

	fn create_images(&self) -> anyhow::Result<Vec<Arc<Image>>>;

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

	fn acquire_next_image(
		&self,
		timeout: u64,
		barrier: ImageAcquisitionBarrier,
	) -> anyhow::Result<AcquiredImage>;

	fn can_present(&self) -> bool {
		false
	}

	fn present(
		&self,
		_graphics_queue: &Arc<logical::Queue>,
		_present_info: command::PresentInfo,
	) -> utility::Result<bool> {
		Ok(false)
	}
}
