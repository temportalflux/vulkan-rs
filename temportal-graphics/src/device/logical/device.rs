use crate::{
	command, flags, pipeline,
	utility::{self, VulkanObject},
};
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

	pub fn allocate_command_buffers(
		&self,
		pool: &command::Pool,
		amount: usize,
	) -> utility::Result<Vec<command::Buffer>> {
		let info = erupt::vk::CommandBufferAllocateInfoBuilder::new()
			.command_pool(*pool.unwrap())
			.level(erupt::vk::CommandBufferLevel::PRIMARY)
			.command_buffer_count(amount as u32)
			.build();
		let alloc_result =
			utility::as_vulkan_error(unsafe { self._internal.allocate_command_buffers(&info) });
		Ok(alloc_result?
			.into_iter()
			.map(|vk_buffer| command::Buffer::from(vk_buffer))
			.collect::<Vec<_>>())
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

	pub fn create_command_pool(
		&self,
		queue_family_index: u32,
	) -> utility::Result<erupt::vk::CommandPool> {
		let info = erupt::vk::CommandPoolCreateInfoBuilder::new()
			.queue_family_index(queue_family_index)
			.build();
		utility::as_vulkan_error(unsafe { self._internal.create_command_pool(&info, None, None) })
	}

	pub fn begin_command_buffer(&self, buffer: &command::Buffer) -> utility::Result<()> {
		let info = erupt::vk::CommandBufferBeginInfoBuilder::new().build();
		utility::as_vulkan_error(unsafe {
			self._internal.begin_command_buffer(*buffer.unwrap(), &info)
		})
	}

	pub fn end_command_buffer(&self, buffer: &command::Buffer) -> utility::Result<()> {
		utility::as_vulkan_error(unsafe { self._internal.end_command_buffer(*buffer.unwrap()) })
	}

	pub fn begin_render_pass(
		&self,
		buffer: &command::Buffer,
		info: erupt::vk::RenderPassBeginInfo,
	) {
		unsafe {
			self._internal.cmd_begin_render_pass(
				*buffer.unwrap(),
				&info,
				erupt::vk::SubpassContents::INLINE,
			)
		};
	}

	pub fn bind_pipeline(
		&self,
		buffer: &command::Buffer,
		pipeline: &pipeline::Pipeline,
		point: flags::PipelineBindPoint,
	) {
		unsafe {
			self._internal
				.cmd_bind_pipeline(*buffer.unwrap(), point, *pipeline.unwrap())
		};
	}

	pub fn end_render_pass(&self, buffer: &command::Buffer) {
		unsafe { self._internal.cmd_end_render_pass(*buffer.unwrap()) };
	}

	pub fn draw(&self, buffer: &command::Buffer, index_count: u32) {
		unsafe {
			self._internal.cmd_draw(
				*buffer.unwrap(),
				index_count,
				/*instance count*/ 1,
				/*fist_index*/ 0,
				/*fist_instance*/ 0,
			)
		};
	}

	pub fn draw_indexed(
		&self,
		buffer: &command::Buffer,
		index_count: u32,
		first_index: u32,
		instance_count: u32,
		first_instance: u32,
		vertex_offset: i32,
	) {
		unsafe {
			self._internal.cmd_draw_indexed(
				*buffer.unwrap(),
				index_count,
				instance_count,
				first_index,
				vertex_offset,
				first_instance,
			)
		};
	}
}
