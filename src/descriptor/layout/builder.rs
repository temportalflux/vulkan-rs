use crate::{
	backend,
	descriptor::layout,
	device::logical,
	flags::{DescriptorKind, ShaderKind},
	utility,
};
use std::sync;

pub struct Builder {
	bindings: Vec<backend::vk::DescriptorSetLayoutBinding>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			bindings: Vec::new(),
		}
	}
}

impl Builder {
	pub fn with_binding(
		mut self,
		binding_number: u32,
		kind: DescriptorKind,
		amount: u32,
		stage: ShaderKind,
	) -> Self {
		self.bindings.push(
			backend::vk::DescriptorSetLayoutBinding::builder()
				.binding(binding_number)
				.descriptor_type(kind)
				.descriptor_count(amount)
				.stage_flags(stage.to_vk())
				.build(),
		);
		self
	}
}

impl Builder {
	/// Creates an [`crate::descriptor::layout::SetLayout`] object, thereby consuming the info.
	pub fn build(self, device: &sync::Arc<logical::Device>) -> utility::Result<layout::SetLayout> {
		use backend::version::DeviceV1_0;
		let create_info = backend::vk::DescriptorSetLayoutCreateInfo::builder()
			.bindings(&self.bindings)
			.build();
		let internal = unsafe { device.create_descriptor_set_layout(&create_info, None) }?;
		Ok(layout::SetLayout::from(device.clone(), internal))
	}
}