use erupt;

#[derive(Debug)]
pub enum Error {
	InvalidInstanceLayer(String),
	InstanceSymbolNotAvailable(),
	VulkanError(erupt::vk::Result),
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
			Error::General(ref e) => e.fmt(f),
		}
	}
}

impl std::error::Error for Error {}

pub fn as_vulkan_error<T>(erupt_result: erupt::utils::VulkanResult<T>) -> Result<T> {
	match erupt_result.result() {
		Ok(v) => Ok(v),
		Err(vk_result) => Err(Error::VulkanError(vk_result)),
	}
}
