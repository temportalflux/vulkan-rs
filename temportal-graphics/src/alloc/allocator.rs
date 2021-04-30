use crate::{
	device::{logical, physical},
	image, instance,
	utility::{self, VulkanObject},
};

pub struct Allocator {
	internal: vk_mem::Allocator,
}

impl Allocator {
	pub fn create(
		instance: &instance::Instance,
		physical: &physical::Device,
		logical: &logical::Device,
	) -> utility::Result<Allocator> {
		let info = vk_mem::AllocatorCreateInfo {
			instance: instance.unwrap().clone(),
			physical_device: *physical.unwrap(),
			device: logical.unwrap().clone(),
			flags: vk_mem::AllocatorCreateFlags::NONE,
			preferred_large_heap_block_size: 0,
			frame_in_use_count: 0,
			heap_size_limits: None,
		};
		Ok(Allocator {
			internal: utility::as_alloc_error(vk_mem::Allocator::new(&info))?,
		})
	}
}

/// A trait exposing the internal value for the wrapped [`vk_mem::Allocator`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<vk_mem::Allocator> for Allocator {
	fn unwrap(&self) -> &vk_mem::Allocator {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut vk_mem::Allocator {
		&mut self.internal
	}
}

impl Drop for Allocator {
	fn drop(&mut self) {
		self.internal.destroy();
	}
}

#[doc(hidden)]
impl image::Owner for Allocator {
	fn destroy(
		&self,
		obj: &image::Image,
		allocation: Option<&vk_mem::Allocation>,
	) -> utility::Result<()> {
		utility::as_alloc_error(
			self.internal
				.destroy_image(*obj.unwrap(), allocation.unwrap()),
		)
	}
}
