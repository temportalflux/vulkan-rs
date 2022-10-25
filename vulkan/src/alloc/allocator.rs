use crate::{
	device::{logical, physical},
	flags::MemoryLocation,
	image, instance,
};
use std::sync;

/// A wrapper for the [`gpu allocator`](gpu-allocator) for handling the allocation of [`graphics objects`](crate::alloc::Object).
pub struct Allocator {
	internal: sync::Mutex<gpu_allocator::vulkan::Allocator>,
	logical: sync::Weak<logical::Device>,
}

impl Allocator {
	/// Creates an allocator for a given vulkan instance and device pair.
	pub fn create(
		instance: &instance::Instance,
		physical: &physical::Device,
		logical: &sync::Arc<logical::Device>,
	) -> anyhow::Result<Allocator> {
		let desc = gpu_allocator::vulkan::AllocatorCreateDesc {
			instance: (**instance).clone(),
			physical_device: **physical,
			device: (**logical).clone(),
			debug_settings: Default::default(),
			buffer_device_address: true, // Ideally, check the BufferDeviceAddressFeatures struct.
		};
		Ok(Allocator {
			internal: sync::Mutex::new(gpu_allocator::vulkan::Allocator::new(&desc)?),
			logical: sync::Arc::downgrade(&logical),
		})
	}

	pub fn logical(&self) -> Option<sync::Arc<logical::Device>> {
		self.logical.upgrade()
	}

	pub fn create_buffer(
		&self,
		name: &str,
		location: MemoryLocation,
		info: &crate::backend::vk::BufferCreateInfo,
	) -> anyhow::Result<(
		crate::backend::vk::Buffer,
		gpu_allocator::vulkan::Allocation,
	)> {
		let device = self.logical().unwrap();
		let buffer = unsafe { device.create_buffer(info, None) }?;
		let requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
		let alloc_desc = gpu_allocator::vulkan::AllocationCreateDesc {
			name,
			requirements,
			location,
			linear: true, // Buffers are always linear
		};
		let allocation = {
			let mut allocator = self.internal.lock().unwrap();
			allocator.allocate(&alloc_desc)?
		};
		unsafe { device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())? };
		Ok((buffer, allocation))
	}

	pub fn create_image(
		&self,
		name: &str,
		location: MemoryLocation,
		info: &crate::backend::vk::ImageCreateInfo,
		is_tiled: bool,
	) -> anyhow::Result<(crate::backend::vk::Image, gpu_allocator::vulkan::Allocation)> {
		let device = self.logical().unwrap();
		let buffer = unsafe { device.create_image(info, None) }?;
		let requirements = unsafe { device.get_image_memory_requirements(buffer) };
		let alloc_desc = gpu_allocator::vulkan::AllocationCreateDesc {
			name,
			requirements,
			location,
			// If the resource is linear (buffer / linear texture) or a regular (tiled) texture.
			linear: !is_tiled,
		};
		let allocation = {
			let mut allocator = self.internal.lock().unwrap();
			allocator.allocate(&alloc_desc)?
		};
		unsafe { device.bind_image_memory(buffer, allocation.memory(), allocation.offset())? };
		Ok((buffer, allocation))
	}

	pub fn destroy_buffer(
		&self,
		buffer: crate::backend::vk::Buffer,
		allocation: gpu_allocator::vulkan::Allocation,
	) -> anyhow::Result<()> {
		let device = self.logical().unwrap();
		if let Ok(mut allocator) = self.internal.lock() {
			allocator.free(allocation)?;
		}
		unsafe { device.destroy_buffer(buffer, None) };
		Ok(())
	}
}

#[doc(hidden)]
impl image::Owner for Allocator {
	fn destroy(
		&self,
		obj: &image::Image,
		allocation: Option<gpu_allocator::vulkan::Allocation>,
	) -> anyhow::Result<()> {
		let device = self.logical().unwrap();
		if let Some(allocation) = allocation {
			if let Ok(mut allocator) = self.internal.lock() {
				allocator.free(allocation)?;
			}
		}
		unsafe { device.destroy_image(**obj, None) };
		Ok(())
	}
}
