use crate::{
	backend,
	descriptor::layout,
	device::logical,
	flags::{DescriptorKind, ShaderKind},
	utility,
};
use std::sync;

/// Prepares the declarations for how descriptor sets will be created, via a [`layout`](layout::SetLayout).
pub struct Builder {
	bindings: Vec<backend::vk::DescriptorSetLayoutBinding>,
	name: String,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			bindings: Vec::new(),
			name: String::new(),
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

impl utility::NameableBuilder for Builder {
	fn set_name(&mut self, name: impl Into<String>) {
		self.name = name.into();
	}

	fn name(&self) -> &String {
		&self.name
	}
}

impl utility::BuildFromDevice for Builder {
	type Output = layout::SetLayout;
	/// Creates an [`crate::descriptor::layout::SetLayout`] object, thereby consuming the info.
	fn build(self, device: &sync::Arc<logical::Device>) -> anyhow::Result<Self::Output> {
		let create_info = backend::vk::DescriptorSetLayoutCreateInfo::builder()
			.bindings(&self.bindings)
			.build();
		let internal = unsafe { device.create_descriptor_set_layout(&create_info, None) }?;
		let layout = layout::SetLayout::from(device.clone(), internal);
		self.set_object_name(device, &layout);
		Ok(layout)
	}
}
