use crate::utility;
use erupt;

/// A wrapper for a [`Vulkan LogicalDevice`](erupt::DeviceLoader),
/// which can send logical commands to the hardware.
pub struct Device {
	_internal: erupt::DeviceLoader,
}

impl Device {
	/// The internal constructor. Users should use [`Info.create_object`](struct.Info.html#method.create_object) to create a vulkan instance.
	pub fn from(internal: erupt::DeviceLoader) -> Device {
		Device {
			_internal: internal,
		}
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::DeviceLoader`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<erupt::DeviceLoader> for Device {
	fn unwrap(&self) -> &erupt::DeviceLoader {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::DeviceLoader {
		&mut self._internal
	}
}

#[doc(hidden)]
impl Device {
	pub fn create_swapchain(
		&self,
		info: erupt::vk::SwapchainCreateInfoKHR,
	) -> Result<erupt::vk::SwapchainKHR, utility::Error> {
		utility::as_vulkan_error(unsafe { self._internal.create_swapchain_khr(&info, None, None) })
	}

	pub fn get_swapchain_images(
		&self,
		swapchain: &erupt::vk::SwapchainKHR,
	) -> Result<Vec<erupt::vk::Image>, utility::Error> {
		utility::as_vulkan_error(unsafe {
			self._internal.get_swapchain_images_khr(*swapchain, None)
		})
	}

	pub fn create_image_view(
		&self,
		info: erupt::vk::ImageViewCreateInfo,
	) -> Result<erupt::vk::ImageView, utility::Error> {
		utility::as_vulkan_error(unsafe { self._internal.create_image_view(&info, None, None) })
	}

	pub fn create_shader_module(
		&self,
		info: erupt::vk::ShaderModuleCreateInfo,
	) -> Result<erupt::vk::ShaderModule, utility::Error> {
		utility::as_vulkan_error(unsafe { self._internal.create_shader_module(&info, None, None) })
	}

	pub fn create_graphics_pipelines(
		&self,
		infos: Vec<erupt::vk::GraphicsPipelineCreateInfoBuilder>,
	) -> Result<Vec<erupt::vk::Pipeline>, utility::Error> {
		utility::as_vulkan_error(unsafe {
			self._internal
				.create_graphics_pipelines(None, &infos[..], None)
		})
	}
}
