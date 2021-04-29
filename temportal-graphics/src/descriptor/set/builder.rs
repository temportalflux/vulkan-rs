use crate::{
	backend,
	descriptor::{layout, pool, set},
	device::logical,
	utility::{self, VulkanObject},
};
use std::rc::Rc;

pub struct Builder {
	layouts: Vec<Rc<layout::SetLayout>>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			layouts: Vec::new(),
		}
	}
}

impl Builder {
	pub fn with_layout(mut self, layout: &Rc<layout::SetLayout>, amount: u32) -> Self {
		for _ in 0..amount {
			self.layouts.push(layout.clone());
		}
		self
	}
}

impl Builder {
	/// Creates an vec of [`crate::descriptor::set::Set`] objects, thereby consuming the info.
	pub fn build(
		self,
		device: &Rc<logical::Device>,
		pool: &Rc<pool::Pool>,
	) -> utility::Result<Vec<set::Set>> {
		use ash::version::DeviceV1_0;
		let set_layouts = self
			.layouts
			.iter()
			.map(|layout| *layout.unwrap())
			.collect::<Vec<_>>();
		let create_info = backend::vk::DescriptorSetAllocateInfo::builder()
			.descriptor_pool(*pool.unwrap())
			.set_layouts(&set_layouts)
			.build();
		Ok(utility::as_vulkan_error(unsafe {
			device.unwrap().allocate_descriptor_sets(&create_info)
		})?
		.into_iter()
		.enumerate()
		.map(|(idx, vk_desc_set)| set::Set::from(self.layouts[idx].clone(), vk_desc_set))
		.collect())
	}
}
