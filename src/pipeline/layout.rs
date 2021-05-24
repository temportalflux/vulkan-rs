use crate::{
	backend, descriptor,
	device::logical,
	utility::{self},
};
use std::sync;

pub struct Builder {
	descriptor_layouts: Vec<sync::Weak<descriptor::SetLayout>>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			descriptor_layouts: Vec::new(),
		}
	}
}

impl Builder {
	pub fn with_descriptors(mut self, layout: &sync::Arc<descriptor::SetLayout>) -> Self {
		self.descriptor_layouts.push(sync::Arc::downgrade(layout));
		self
	}
}

impl Builder {
	pub fn build(self, device: sync::Arc<logical::Device>) -> utility::Result<Layout> {
		use backend::version::DeviceV1_0;

		let vk_descriptor_layouts = self
			.descriptor_layouts
			.iter()
			.filter_map(|layout| layout.upgrade().map(|rc| **rc))
			.collect::<Vec<_>>();
		let vk_info = backend::vk::PipelineLayoutCreateInfo::builder()
			.set_layouts(&vk_descriptor_layouts[..])
			.build();

		Ok(Layout {
			internal: unsafe { device.create_pipeline_layout(&vk_info, None) }?,
			device,
		})
	}
}

pub struct Layout {
	internal: backend::vk::PipelineLayout,
	device: sync::Arc<logical::Device>,
}

impl Layout {
	pub fn builder() -> Builder {
		Builder::default()
	}
}

impl std::ops::Deref for Layout {
	type Target = backend::vk::PipelineLayout;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Layout {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.destroy_pipeline_layout(self.internal, None) };
	}
}
