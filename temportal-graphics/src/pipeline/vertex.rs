use crate::{backend, flags, pipeline::vertex, utility::VulkanInfo};

pub struct Layout {
	bindings: Vec<backend::vk::VertexInputBindingDescription>,
	attributes: Vec<backend::vk::VertexInputAttributeDescription>,
}

impl Default for Layout {
	fn default() -> Layout {
		Layout {
			bindings: Vec::new(),
			attributes: Vec::new(),
		}
	}
}

impl Layout {
	pub fn with_object<T>(mut self, binding_index: usize, rate: flags::VertexInputRate) -> Self
	where
		T: vertex::Object,
	{
		self.bindings.push(
			backend::vk::VertexInputBindingDescription::builder()
				.binding(binding_index as u32)
				.stride(std::mem::size_of::<T>() as u32)
				.input_rate(rate)
				.build(),
		);
		for attriute in T::attributes() {
			self.attributes.push(
				backend::vk::VertexInputAttributeDescription::builder()
					.binding(binding_index as u32)
					.location(self.attributes.len() as u32)
					.offset(attriute.offset as u32)
					.format(attriute.format)
					.build(),
			);
		}
		self
	}
}

impl VulkanInfo<backend::vk::PipelineVertexInputStateCreateInfo> for Layout {
	fn to_vk(&self) -> backend::vk::PipelineVertexInputStateCreateInfo {
		backend::vk::PipelineVertexInputStateCreateInfo::builder()
			.vertex_binding_descriptions(&self.bindings[..])
			.vertex_attribute_descriptions(&self.attributes[..])
			.build()
	}
}

pub struct Attribute {
	pub offset: usize,
	pub format: flags::Format,
}

pub trait Object: Sized {
	fn attributes() -> Vec<Attribute>;
}
