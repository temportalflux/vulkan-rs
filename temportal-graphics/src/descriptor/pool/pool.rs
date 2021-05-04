use crate::{
	backend, descriptor,
	device::logical,
	utility::{self, VulkanObject},
};
use std::sync;

pub struct Pool {
	owned_sets: Vec<sync::Arc<descriptor::Set>>,
	internal: backend::vk::DescriptorPool,
	device: sync::Arc<logical::Device>,
}

impl Pool {
	pub fn builder() -> descriptor::pool::Builder {
		descriptor::pool::Builder::default()
	}

	pub fn from(device: sync::Arc<logical::Device>, internal: backend::vk::DescriptorPool) -> Pool {
		Pool {
			device,
			internal,
			owned_sets: Vec::new(),
		}
	}

	pub fn allocate_descriptor_sets(
		&mut self,
		layouts: &Vec<sync::Arc<descriptor::SetLayout>>,
	) -> utility::Result<Vec<sync::Weak<descriptor::Set>>> {
		use ash::version::DeviceV1_0;
		let set_layouts = layouts
			.iter()
			.map(|layout| *layout.unwrap())
			.collect::<Vec<_>>();
		let create_info = backend::vk::DescriptorSetAllocateInfo::builder()
			.descriptor_pool(*self.unwrap())
			.set_layouts(&set_layouts)
			.build();
		let raw_sets = utility::as_vulkan_error(unsafe {
			self.device.unwrap().allocate_descriptor_sets(&create_info)
		})?;
		Ok(raw_sets
			.into_iter()
			.enumerate()
			.map(|(idx, vk_desc_set)| {
				let set = sync::Arc::new(descriptor::Set::from(layouts[idx].clone(), vk_desc_set));
				self.owned_sets.push(set.clone());
				sync::Arc::downgrade(&set)
			})
			.collect())
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::DescriptorPool`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::DescriptorPool> for Pool {
	fn unwrap(&self) -> &backend::vk::DescriptorPool {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::DescriptorPool {
		&mut self.internal
	}
}

impl Drop for Pool {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.unwrap()
				.destroy_descriptor_pool(self.internal, None);
		}
	}
}
