use crate::{command, image_view::View};

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

pub trait Swapchain {
	fn get_image_views(&self) -> anyhow::Result<Vec<View>>;
	fn acquire_next_image(
		&self,
		timeout: u64,
		barrier: ImageAcquisitionBarrier,
	) -> anyhow::Result<AcquiredImage>;
}
