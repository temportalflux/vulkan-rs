use crate::{
	backend,
	device::logical,
	utility::{self, VulkanObject},
};

use std::rc::Rc;

pub struct Layout {
	internal: backend::vk::PipelineLayout,
	device: Rc<logical::Device>,
}

impl Layout {
	pub fn create(device: Rc<logical::Device>) -> utility::Result<Layout> {
		use backend::version::DeviceV1_0;
		let vk_info = backend::vk::PipelineLayoutCreateInfo::builder().build();
		let internal = utility::as_vulkan_error(unsafe {
			device.unwrap().create_pipeline_layout(&vk_info, None)
		})?;
		Ok(Layout { device, internal })
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
