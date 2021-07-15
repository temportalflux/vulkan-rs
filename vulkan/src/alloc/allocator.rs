use crate::{
	device::{logical, physical},
	image, instance,
	utility::{self},
};
use std::sync;

/// A wrapper for the [`vulkan memory allocator`](vk_mem) for handling the allocation of [`graphics objects`](crate::alloc::Object).
pub struct Allocator {
	internal: vk_mem::Allocator,
	logical: sync::Weak<logical::Device>,
}

impl Allocator {
	/// Creates an allocator for a given vulkan instance and device pair.
	pub fn create(
		instance: &instance::Instance,
		physical: &physical::Device,
		logical: &sync::Arc<logical::Device>,
	) -> utility::Result<Allocator> {
		let info = vk_mem::AllocatorCreateInfo {
			instance: (**instance).clone(),
			physical_device: **physical,
			device: (**logical).clone(),
			flags: vk_mem::AllocatorCreateFlags::NONE,
			preferred_large_heap_block_size: 0,
			frame_in_use_count: 0,
			heap_size_limits: None,
		};
		Ok(Allocator {
			internal: vk_mem::Allocator::new(&info)?,
			logical: sync::Arc::downgrade(&logical),
		})
	}

	pub fn logical(&self) -> Option<sync::Arc<logical::Device>> {
		self.logical.upgrade()
	}
}

/// A trait exposing the internal value for the wrapped [`vk_mem::Allocator`].
/// Crates using `vulkan_rs` should NOT use this.
impl std::ops::Deref for Allocator {
	type Target = vk_mem::Allocator;
	fn deref(&self) -> &Self::Target {
		&self.internal
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
		Ok(self.internal.destroy_image(**obj, allocation.unwrap())?)
	}
}
