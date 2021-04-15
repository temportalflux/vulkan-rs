use erupt;

#[derive(Debug)]
pub enum Error {
	InvalidInstanceLayer(String),
	VulkanError(erupt::vk::Result),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Error::InvalidInstanceLayer(ref layer_name) => {
				write!(f, "Invalid vulkan instance layer: {}", layer_name)
			}
			Error::VulkanError(ref vk_result) => vk_result.fmt(f),
		}
	}
}

impl std::error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::InvalidInstanceLayer(ref layer_name) => layer_name.as_str(),
			Error::VulkanError(ref vk_result) => vk_result.description(),
		}
	}
}

pub fn as_vulkan_error<T>(erupt_result: erupt::utils::VulkanResult<T>) -> Result<T, Error> {
	match erupt_result.result() {
		Ok(v) => Ok(v),
		Err(vk_result) => Err(Error::VulkanError(vk_result)),
	}
}
