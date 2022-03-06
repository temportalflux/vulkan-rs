use super::super::{AcquiredImage, ImageAcquisitionBarrier, Swapchain as SwapchainTrait};
use crate::{device::logical, image::Image, structs};
use std::sync::Arc;

pub struct Swapchain {
	device: Arc<logical::Device>,
	images: Vec<Arc<Image>>,
	image_extent: structs::Extent2D,
}

impl SwapchainTrait for Swapchain {
	fn device(&self) -> &Arc<logical::Device> {
		&self.device
	}

	fn image_count(&self) -> usize {
		0
	}

	fn image_extent(&self) -> &structs::Extent2D {
		&self.image_extent
	}

	fn create_images(&self) -> anyhow::Result<Vec<Arc<Image>>> {
		Ok(self.images.clone())
	}

	/// Determines the index of the image to render the next frame to.
	fn acquire_next_image(
		&self,
		_timeout: u64,
		_barrier: ImageAcquisitionBarrier,
	) -> anyhow::Result<AcquiredImage> {
		Ok(AcquiredImage::Available(0))
	}
}
