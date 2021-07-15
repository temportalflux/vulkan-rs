use crate::{
	backend,
	descriptor::layout::SetLayout,
	device::logical,
	utility,
};
use std::sync;

/// The builder for a pipeline [`Layout`].
pub struct Builder {
	descriptor_layouts: Vec<sync::Weak<SetLayout>>,
	name: Option<String>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			descriptor_layouts: Vec::new(),
			name: None,
		}
	}
}

impl Builder {
	pub fn with_descriptors(mut self, layout: &sync::Arc<SetLayout>) -> Self {
		self.descriptor_layouts.push(sync::Arc::downgrade(layout));
		self
	}
}

impl utility::NameableBuilder for Builder {
	fn with_optname(mut self, name: Option<String>) -> Self {
		self.name = name;
		self
	}

	fn name(&self) -> &Option<String> {
		&self.name
	}
}

impl utility::BuildFromDevice for Builder {
	type Output = Layout;
	fn build(self, device: &sync::Arc<logical::Device>) -> utility::Result<Self::Output> {
		use backend::version::DeviceV1_0;

		let vk_descriptor_layouts = self
			.descriptor_layouts
			.iter()
			.filter_map(|layout| layout.upgrade().map(|rc| **rc))
			.collect::<Vec<_>>();
		let vk_info = backend::vk::PipelineLayoutCreateInfo::builder()
			.set_layouts(&vk_descriptor_layouts[..])
			.build();
		let layout = Layout {
			internal: unsafe { device.create_pipeline_layout(&vk_info, None) }?,
			device: device.clone(),
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

impl utility::HandledObject for Layout {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::PipelineLayout as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
