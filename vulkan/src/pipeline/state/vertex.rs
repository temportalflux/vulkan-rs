//! Configuring how vertex data is laid out in memory.

use crate::{backend, flags};

pub use memoffset::{offset_of, offset_of_tuple};

/// A description of how vertex attributes in a shader/pipeline are laid out.
/// Leverages [`Object`] which users implement to describe a struct of data.
#[derive(Debug, Default)]
pub struct Layout {
	bindings: Vec<backend::vk::VertexInputBindingDescription>,
	attributes: Vec<backend::vk::VertexInputAttributeDescription>,
}

impl Layout {
	pub fn with_object<T>(mut self, binding_index: usize, rate: flags::VertexInputRate) -> Self
	where
		T: Object,
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

	pub(crate) fn as_vk(&self) -> backend::vk::PipelineVertexInputStateCreateInfo {
		backend::vk::PipelineVertexInputStateCreateInfo::builder()
			.vertex_binding_descriptions(&self.bindings[..])
			.vertex_attribute_descriptions(&self.attributes[..])
			.build()
	}
}

/// An individual attribute field in a vertex shader.
///
/// One example of a vertex shader attribute: `layout(location = 1) in vec4 in_color;`
#[derive(Debug)]
pub struct Attribute {
	pub offset: usize,
	pub format: flags::format::Format,
}

/// A description of a data struct which contains vertex data.
///
/// See [`crate::vertex_object`] for a less-verbose implementation.
pub trait Object: Sized {
	fn attributes() -> Vec<Attribute>;
}
