use crate::{backend, descriptor::layout::SetLayout, device::logical, utility};
use std::sync;

/// The builder for a pipeline [`Layout`].
#[derive(Clone)]
pub struct Builder {
	descriptor_layouts: Vec<sync::Weak<SetLayout>>,
	push_constant_ranges: Vec<super::PushConstantRange>,
	name: String,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			descriptor_layouts: Vec::new(),
			push_constant_ranges: Vec::new(),
			name: String::new(),
		}
	}
}

impl Builder {
	pub fn clear_descriptor_layouts(&mut self) {
		self.descriptor_layouts.clear();
	}

	pub fn with_descriptors(mut self, layout: &sync::Arc<SetLayout>) -> Self {
		self.add_descriptor_layout(layout);
		self
	}

	pub fn add_descriptor_layout(&mut self, layout: &sync::Arc<SetLayout>) {
		self.descriptor_layouts.push(sync::Arc::downgrade(layout));
	}

	pub fn with_push_constant_range(mut self, range: super::PushConstantRange) -> Self {
		self.add_push_constant_range(range);
		self
	}

	pub fn add_push_constant_range(&mut self, range: super::PushConstantRange) {
		self.push_constant_ranges.push(range);
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
	type Output = Layout;
	fn build(self, device: &sync::Arc<logical::Device>) -> anyhow::Result<Self::Output> {
		let vk_descriptor_layouts = self
			.descriptor_layouts
			.iter()
			.filter_map(|layout| layout.upgrade().map(|rc| **rc))
			.collect::<Vec<_>>();
		let vk_ranges = self
			.push_constant_ranges
			.clone()
			.into_iter()
			.map(|range| range.into())
			.collect::<Vec<backend::vk::PushConstantRange>>();
		let vk_info = backend::vk::PipelineLayoutCreateInfo::builder()
			.set_layouts(&vk_descriptor_layouts[..])
			.push_constant_ranges(&vk_ranges[..])
			.build();
		let layout = Layout {
			internal: unsafe { device.create_pipeline_layout(&vk_info, None) }?,
			device: device.clone(),
			name: self.name.clone(),
		};
		self.set_object_name(device, &layout);
		Ok(layout)
	}
}

/// A pipeline layout contains information about a pipeline's descriptor sets and push constants.
/// These layouts can be empty if there are no descriptors or push constants,
/// but more often than not you will have at least 1 [`descriptor set layout`](SetLayout) that is bound.
///
/// Equivalent to [`VkPipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayout.html).
pub struct Layout {
	internal: backend::vk::PipelineLayout,
	device: sync::Arc<logical::Device>,
	name: String,
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
		log::debug!(
			target: crate::LOG,
			"Dropping PipelineLayout: {:?}",
			self.name
		);
		unsafe { self.device.destroy_pipeline_layout(self.internal, None) };
	}
}

impl utility::HandledObject for Layout {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::PipelineLayout as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
