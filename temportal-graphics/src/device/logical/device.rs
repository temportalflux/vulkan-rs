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
	) -> utility::Result<erupt::vk::SwapchainKHR> {
		utility::as_vulkan_error(unsafe { self._internal.create_swapchain_khr(&info, None, None) })
	}

	pub fn get_swapchain_images(
		&self,
		swapchain: &erupt::vk::SwapchainKHR,
	) -> utility::Result<Vec<erupt::vk::Image>> {
		utility::as_vulkan_error(unsafe {
			self._internal.get_swapchain_images_khr(*swapchain, None)
		})
	}

	pub fn create_image_view(
		&self,
		info: erupt::vk::ImageViewCreateInfo,
	) -> utility::Result<erupt::vk::ImageView> {
		utility::as_vulkan_error(unsafe { self._internal.create_image_view(&info, None, None) })
	}

	pub fn create_shader_module(
		&self,
		info: erupt::vk::ShaderModuleCreateInfo,
	) -> utility::Result<erupt::vk::ShaderModule> {
		utility::as_vulkan_error(unsafe { self._internal.create_shader_module(&info, None, None) })
	}

	pub fn create_pipeline_layout(
		&self,
		info: erupt::vk::PipelineLayoutCreateInfo,
	) -> utility::Result<erupt::vk::PipelineLayout> {
		utility::as_vulkan_error(unsafe {
			self._internal.create_pipeline_layout(&info, None, None)
		})
	}

	pub fn create_graphics_pipelines(
		&self,
		infos: &[erupt::vk::GraphicsPipelineCreateInfoBuilder<'_>],
	) -> utility::Result<Vec<erupt::vk::Pipeline>> {
		utility::as_vulkan_error(unsafe {
			self._internal.create_graphics_pipelines(None, infos, None)
		})
	}

	pub fn create_render_pass(
		&self,
		info: erupt::vk::RenderPassCreateInfo,
	) -> utility::Result<erupt::vk::RenderPass> {
		utility::as_vulkan_error(unsafe { self._internal.create_render_pass(&info, None, None) })
	}

	pub fn create_framebuffer(
		&self,
		info: erupt::vk::FramebufferCreateInfo,
	) -> utility::Result<erupt::vk::Framebuffer> {
		utility::as_vulkan_error(unsafe { self._internal.create_framebuffer(&info, None, None) })
	}
}
