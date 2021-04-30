use crate::{
	backend, descriptor,
	device::logical,
	utility::{self, VulkanInfo, VulkanObject},
};
use std::rc::{Rc, Weak};

pub struct Builder {
	descriptor_layouts: Vec<Weak<descriptor::SetLayout>>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			descriptor_layouts: Vec::new(),
		}
	}
}

impl Builder {
	pub fn with_descriptors(mut self, layout: &Rc<descriptor::SetLayout>) -> Self {
		self.descriptor_layouts.push(Rc::downgrade(layout));
		self
	}
}

impl VulkanInfo<backend::vk::PipelineLayoutCreateInfo> for Builder {
	fn to_vk(&self) -> backend::vk::PipelineLayoutCreateInfo {
		let desc_layouts = self
			.descriptor_layouts
			.iter()
			.filter_map(|layout| layout.upgrade().map(|rc| *rc.unwrap()))
			.collect::<Vec<_>>();
		backend::vk::PipelineLayoutCreateInfo::builder()
			.set_layouts(&desc_layouts[..])
			.build()
	}
}

impl Builder {
	pub fn build(self, device: Rc<logical::Device>) -> utility::Result<Layout> {
		use backend::version::DeviceV1_0;
		let vk_info = self.to_vk();
		let internal = utility::as_vulkan_error(unsafe {
			device.unwrap().create_pipeline_layout(&vk_info, None)
		})?;
		Ok(Layout { device, internal })
	}
}

pub struct Layout {
	internal: backend::vk::PipelineLayout,
	device: Rc<logical::Device>,
}

impl Layout {
	pub fn builder() -> Builder {
		Builder::default()
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::PipelineLayout`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::PipelineLayout> for Layout {
	fn unwrap(&self) -> &backend::vk::PipelineLayout {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::PipelineLayout {
		&mut self.internal
	}
}

impl Drop for Layout {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.unwrap()
				.destroy_pipeline_layout(self.internal, None)
		};
	}
}
