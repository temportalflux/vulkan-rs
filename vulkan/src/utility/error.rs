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

impl From<backend::vk::Result> for Error {
	fn from(err: backend::vk::Result) -> Error {
		match err {
			backend::vk::Result::SUBOPTIMAL_KHR | backend::vk::Result::ERROR_OUT_OF_DATE_KHR => {
				Error::RequiresRenderChainUpdate()
			}
			_ => Error::VulkanError(err),
		}
	}
}

impl From<vk_mem::Error> for Error {
	fn from(err: vk_mem::Error) -> Error {
		Error::AllocatorError(err)
	}
}
