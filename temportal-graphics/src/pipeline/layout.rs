use crate::{
	device::logical,
	utility::{self, VulkanObject},
};
use erupt;
use std::rc::Rc;

pub struct Layout {
	_device: Rc<logical::Device>,
	_internal: erupt::vk::PipelineLayout,
}

impl Layout {
	pub fn create(_device: Rc<logical::Device>) -> utility::Result<Layout> {
		let vk_info = erupt::vk::PipelineLayoutCreateInfoBuilder::new().build();
		let _internal = _device.create_pipeline_layout(vk_info)?;
		Ok(Layout { _device, _internal })
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::PipelineLayout`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::PipelineLayout> for Layout {
	fn unwrap(&self) -> &erupt::vk::PipelineLayout {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::PipelineLayout {
		&mut self._internal
	}
}

impl Drop for Layout {
	fn drop(&mut self) {
		self._device.destroy_pipeline_layout(self._internal)
	}
}
