use crate::utility::VulkanObject;
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
impl VulkanObject<erupt::DeviceLoader> for Device {
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
	) -> erupt::vk::SwapchainKHR {
		unsafe { self._internal.create_swapchain_khr(&info, None, None) }.unwrap()
	}

	pub fn get_swapchain_images(
		&self,
		swapchain: &erupt::vk::SwapchainKHR,
	) -> Vec<erupt::vk::Image> {
		unsafe { self._internal.get_swapchain_images_khr(*swapchain, None) }.unwrap()
	}

	pub fn create_image_view(&self, info: erupt::vk::ImageViewCreateInfo) -> erupt::vk::ImageView {
		unsafe { self._internal.create_image_view(&info, None, None) }.unwrap()
	}

	pub fn create_shader_module(
		&self,
		info: erupt::vk::ShaderModuleCreateInfo,
	) -> erupt::vk::ShaderModule {
		unsafe { self._internal.create_shader_module(&info, None, None) }.unwrap()
	}

	pub fn create_graphics_pipelines(
		&self,
		infos: Vec<erupt::vk::GraphicsPipelineCreateInfoBuilder>,
	) -> Vec<erupt::vk::Pipeline> {
		unsafe {
			self._internal
				.create_graphics_pipelines(None, &infos[..], None)
		}
		.unwrap()
	}
}
