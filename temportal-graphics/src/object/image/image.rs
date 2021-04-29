use crate::{
	backend, image,
	structs::Extent3D,
	utility::{self, VulkanObject},
};

use std::rc::Rc;

pub trait Owner {
	fn destroy(&self, obj: &Image, allocation: Option<&vk_mem::Allocation>) -> utility::Result<()>;
}

/// An handle representing image data stored on the [`GPU`](crate::device::physical::Device),
/// including any created by the [`Swapchain`](crate::device::swapchain::Swapchain).
pub struct Image {
	image_info: Option<image::Builder>,
	allocation_info: Option<vk_mem::AllocationInfo>,
	allocation_handle: Option<vk_mem::Allocation>,
	internal: backend::vk::Image,
	owner: Option<Rc<dyn Owner>>, // empty for images created from the swapchain
}

impl Image {
	pub fn from_swapchain(internal: backend::vk::Image) -> Image {
		Image {
			owner: None,
			internal,
			allocation_handle: None,
			allocation_info: None,
			image_info: None,
		}
	}
	pub fn new(
		owner: Rc<dyn Owner>,
		internal: backend::vk::Image,
		allocation_handle: Option<vk_mem::Allocation>,
		allocation_info: Option<vk_mem::AllocationInfo>,
		image_info: Option<image::Builder>,
	) -> Image {
		Image {
			owner: Some(owner),
			internal,
			allocation_handle,
			allocation_info,
			image_info,
		}
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Image`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::Image> for Image {
	fn unwrap(&self) -> &backend::vk::Image {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Image {
		&mut self.internal
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		if let Some(owner) = &self.owner {
			owner
				.destroy(self, self.allocation_handle.as_ref())
				.unwrap();
		}
	}
}

impl Image {
	pub fn image_size(&self) -> &Extent3D {
		&self.image_info.as_ref().unwrap().extent
	}

	pub fn memory_size(&self) -> usize {
		self.allocation_info.as_ref().unwrap().get_size()
	}
}
