use crate::{
	backend,
	descriptor::pool,
	device::logical,
	flags::DescriptorKind,
	utility::{self, VulkanInfo, VulkanObject},
};
use std::sync;

pub struct Builder {
	/// The maximum number of sets ever allowed to be allocated from the pool.
	max_sets: u32,
	descriptors: Vec<backend::vk::DescriptorPoolSize>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			max_sets: 0,
			descriptors: Vec::new(),
		}
	}
}

impl Builder {
	/// Sets the maximum number of sets that can be created from the pool.
	pub fn with_total_set_count(mut self, max_set_count: u32) -> Self {
		self.max_sets = max_set_count;
		self
	}

	/// Denotates that the pool should create `amount` number of descriptors of a certain kind.
	/// This descriptor amount is shared between all descriptor sets allocated from the pool.
	pub fn with_descriptor(mut self, kind: DescriptorKind, amount: u32) -> Self {
		self.descriptors.push(backend::vk::DescriptorPoolSize {
			ty: kind,
			descriptor_count: amount,
		});
		self
	}
}

impl VulkanInfo<backend::vk::DescriptorPoolCreateInfo> for Builder {
	/// Converts the [`Builder`] into the [`backend::vk::ImageCreateInfo`] struct
	/// used to create a [`crate::image::Image`].
	fn to_vk(&self) -> backend::vk::DescriptorPoolCreateInfo {
		backend::vk::DescriptorPoolCreateInfo::builder()
			.max_sets(self.max_sets)
			.pool_sizes(&self.descriptors)
			.build()
	}
}

impl Builder {
	/// Creates an [`crate::descriptor::pool::Pool`] object, thereby consuming the info.
	pub fn build(self, device: &sync::Arc<logical::Device>) -> utility::Result<pool::Pool> {
		use backend::version::DeviceV1_0;
		let create_info = self.to_vk();
		let internal = utility::as_vulkan_error(unsafe {
			device.unwrap().create_descriptor_pool(&create_info, None)
		})?;
		Ok(pool::Pool::from(device.clone(), internal))
	}
}
