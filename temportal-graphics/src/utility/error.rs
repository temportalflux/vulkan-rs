use crate::backend;

#[derive(Debug)]
pub enum Error {
	InvalidInstanceLayer(String),
	InstanceSymbolNotAvailable(),
	VulkanError(backend::vk::Result),
	AllocatorError(vk_mem::error::Error),
	RequiresRenderChainUpdate(),
	General(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Error::InvalidInstanceLayer(ref layer_name) => {
				write!(f, "Invalid vulkan instance layer: {}", layer_name)
			}
			Error::InstanceSymbolNotAvailable() => write!(f, "Instance symbol not available"),
			Error::VulkanError(ref vk_result) => vk_result.fmt(f),
			Error::AllocatorError(ref vk_mem_error) => vk_mem_error.fmt(f),
			Error::RequiresRenderChainUpdate() => write!(f, "Render chain is out of date"),
			Error::General(ref e) => e.fmt(f),
		}
	}
}

impl std::error::Error for Error {}

pub fn as_vulkan_error<T>(vk_result: backend::prelude::VkResult<T>) -> Result<T> {
	match vk_result {
		Ok(v) => Ok(v),
		Err(vk_result) => match vk_result {
			backend::vk::Result::SUBOPTIMAL_KHR | backend::vk::Result::ERROR_OUT_OF_DATE_KHR => {
				Err(Error::RequiresRenderChainUpdate())
			}
			_ => Err(Error::VulkanError(vk_result)),
		},
	}
}

pub fn as_alloc_error<T>(result: vk_mem::Result<T>) -> Result<T> {
	result.or_else(|err| Err(Error::AllocatorError(err)))
}
