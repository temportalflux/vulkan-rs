use crate::{backend, descriptor, utility::VulkanObject};
use std::rc::Rc;

pub struct Set {
	internal: backend::vk::DescriptorSet,
	_layout: Rc<descriptor::SetLayout>,
}

impl Set {
	pub fn from(layout: Rc<descriptor::SetLayout>, internal: backend::vk::DescriptorSet) -> Set {
		Set {
			_layout: layout,
			internal,
		}
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::DescriptorSetLayout`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::DescriptorSet> for Set {
	fn unwrap(&self) -> &backend::vk::DescriptorSet {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::DescriptorSet {
		&mut self.internal
	}
}
