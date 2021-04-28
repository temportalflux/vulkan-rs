use crate::{
	backend, command,
	device::{logical, swapchain::Swapchain},
	flags, pipeline,
	utility::{self, VulkanInfo, VulkanObject},
};

use std::rc::Rc;

/// A wrapper for a [`Vulkan LogicalDevice`](backend::DeviceLoader),
/// which can send logical commands to the hardware.
pub struct Device {
	_internal: backend::DeviceLoader,
}

impl Device {
	/// The internal constructor. Users should use [`Info.create_object`](struct.Info.html#method.create_object) to create a vulkan instance.
	pub fn from(internal: backend::DeviceLoader) -> Device {
		Device {
			_internal: internal,
		}
	}

	pub fn get_queue(device: &Rc<Self>, queue_family_index: usize) -> logical::Queue {
		let vk = device.get_device_queue(queue_family_index as u32);
		logical::Queue::from(device.clone(), vk, queue_family_index)
	}

	pub fn allocate_command_buffers(
		device: &Rc<Self>,
		pool: &command::Pool,
		amount: usize,
	) -> utility::Result<Vec<command::Buffer>> {
		let info = backend::vk::CommandBufferAllocateInfoBuilder::new()
			.command_pool(*pool.unwrap())
			.level(backend::vk::CommandBufferLevel::PRIMARY)
			.command_buffer_count(amount as u32)
			.build();
		let alloc_result =
			utility::as_vulkan_error(unsafe { device._internal.allocate_command_buffers(&info) });
		Ok(alloc_result?
			.into_iter()
			.map(|vk_buffer| command::Buffer::from(device.clone(), vk_buffer))
			.collect::<Vec<_>>())
	}

	pub fn create_semaphores(
		device: &Rc<Self>,
		count: usize,
	) -> utility::Result<Vec<command::Semaphore>> {
		let mut vec: Vec<command::Semaphore> = Vec::new();
		let info = backend::vk::SemaphoreCreateInfoBuilder::new().build();
		for _ in 0..count {
			let vk_semaphore = utility::as_vulkan_error(unsafe {
				device._internal.create_semaphore(&info, None, None)
			})?;
			vec.push(command::Semaphore::from(device.clone(), vk_semaphore));
		}
		Ok(vec)
	}

	pub fn create_fences(
		device: &Rc<Self>,
		count: usize,
		state: flags::FenceState,
	) -> utility::Result<Vec<command::Fence>> {
		let mut vec: Vec<command::Fence> = Vec::new();
		let info = backend::vk::FenceCreateInfoBuilder::new()
			.flags(state)
			.build();
		for _ in 0..count {
			let vk_fence = utility::as_vulkan_error(unsafe {
				device._internal.create_fence(&info, None, None)
			})?;
			vec.push(command::Fence::from(device.clone(), vk_fence));
		}
		Ok(vec)
	}

	pub fn wait_for(
		&self,
		fence: &command::Fence,
		wait_for_all: bool,
		timeout: u64,
	) -> utility::Result<()> {
		let fences = [*fence.unwrap()];
		utility::as_vulkan_error(unsafe {
			self._internal
				.wait_for_fences(&fences, wait_for_all, timeout)
		})
	}

	pub fn acquire_next_image(
		&self,
		swapchain: &Swapchain,
		timeout: u64,
		semaphore: Option<&command::Semaphore>,
		fence: Option<&command::Fence>,
	) -> utility::Result<usize> {
		utility::as_vulkan_error(unsafe {
			self._internal.acquire_next_image_khr(
				*swapchain.unwrap(),
				timeout,
				semaphore.map(|s| *s.unwrap()),
				fence.map(|s| *s.unwrap()),
				None,
			)
		})
		.map(|i| i as usize)
	}

	pub fn reset_fences(&self, fences: &[&command::Fence]) -> utility::Result<()> {
		let fences = fences.iter().map(|f| *f.unwrap()).collect::<Vec<_>>();
		utility::as_vulkan_error(unsafe { self._internal.reset_fences(&fences[..]) })
	}

	pub fn submit(
		&self,
		queue: &logical::Queue,
		infos: Vec<command::SubmitInfo>,
		signal_fence_when_complete: Option<&command::Fence>,
	) -> utility::Result<()> {
		let infos = infos
			.iter()
			.map(|info| info.to_vk().into_builder())
			.collect::<Vec<_>>();
		utility::as_vulkan_error(unsafe {
			self._internal.queue_submit(
				*queue.unwrap(),
				&infos,
				signal_fence_when_complete.map(|f| *f.unwrap()),
			)
		})
	}

	pub fn present(
		&self,
		queue: &logical::Queue,
		info: command::PresentInfo,
	) -> utility::Result<()> {
		let vk_info = info.to_vk();
		utility::as_vulkan_error(unsafe {
			self._internal.queue_present_khr(*queue.unwrap(), &vk_info)
		})
	}

	pub fn wait_until_idle(&self) -> utility::Result<()> {
		utility::as_vulkan_error(unsafe { self._internal.device_wait_idle() })
	}
}

/// A trait exposing the internal value for the wrapped [`backend::DeviceLoader`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<backend::DeviceLoader> for Device {
	fn unwrap(&self) -> &backend::DeviceLoader {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::DeviceLoader {
		&mut self._internal
	}
}

impl Drop for Device {
	fn drop(&mut self) {
		unsafe {
			self._internal.destroy_device(None);
		}
	}
}

#[doc(hidden)]
impl Device {
	pub fn get_device_queue(&self, queue_family_index: u32) -> backend::vk::Queue {
		unsafe {
			self._internal
				.get_device_queue(queue_family_index, /*queue index*/ 0, None)
		}
	}

	pub fn create_swapchain(
		&self,
		info: backend::vk::SwapchainCreateInfoKHR,
	) -> utility::Result<backend::vk::SwapchainKHR> {
		utility::as_vulkan_error(unsafe { self._internal.create_swapchain_khr(&info, None, None) })
	}

	pub fn destroy_swapchain(&self, value: backend::vk::SwapchainKHR) {
		unsafe { self._internal.destroy_swapchain_khr(Some(value), None) };
	}

	pub fn get_swapchain_images(
		&self,
		swapchain: &backend::vk::SwapchainKHR,
	) -> utility::Result<Vec<backend::vk::Image>> {
		utility::as_vulkan_error(unsafe {
			self._internal.get_swapchain_images_khr(*swapchain, None)
		})
	}

	pub fn destroy_image(&self, value: backend::vk::Image) {
		unsafe { self._internal.destroy_image(Some(value), None) };
	}

	pub fn create_image_view(
		&self,
		info: backend::vk::ImageViewCreateInfo,
	) -> utility::Result<backend::vk::ImageView> {
		utility::as_vulkan_error(unsafe { self._internal.create_image_view(&info, None, None) })
	}

	pub fn destroy_image_view(&self, value: backend::vk::ImageView) {
		unsafe { self._internal.destroy_image_view(Some(value), None) };
	}

	pub fn create_shader_module(
		&self,
		info: backend::vk::ShaderModuleCreateInfo,
	) -> utility::Result<backend::vk::ShaderModule> {
		utility::as_vulkan_error(unsafe { self._internal.create_shader_module(&info, None, None) })
	}

	pub fn destroy_shader_module(&self, value: backend::vk::ShaderModule) {
		unsafe { self._internal.destroy_shader_module(Some(value), None) };
	}

	pub fn create_pipeline_layout(
		&self,
		info: backend::vk::PipelineLayoutCreateInfo,
	) -> utility::Result<backend::vk::PipelineLayout> {
		utility::as_vulkan_error(unsafe {
			self._internal.create_pipeline_layout(&info, None, None)
		})
	}

	pub fn destroy_pipeline_layout(&self, value: backend::vk::PipelineLayout) {
		unsafe { self._internal.destroy_pipeline_layout(Some(value), None) };
	}

	pub fn create_graphics_pipelines(
		&self,
		infos: &[backend::vk::GraphicsPipelineCreateInfoBuilder<'_>],
	) -> utility::Result<Vec<backend::vk::Pipeline>> {
		utility::as_vulkan_error(unsafe {
			self._internal.create_graphics_pipelines(None, infos, None)
		})
	}

	pub fn destroy_pipeline(&self, value: backend::vk::Pipeline) {
		unsafe { self._internal.destroy_pipeline(Some(value), None) };
	}

	pub fn create_render_pass(
		&self,
		info: backend::vk::RenderPassCreateInfo,
	) -> utility::Result<backend::vk::RenderPass> {
		utility::as_vulkan_error(unsafe { self._internal.create_render_pass(&info, None, None) })
	}

	pub fn destroy_render_pass(&self, value: backend::vk::RenderPass) {
		unsafe { self._internal.destroy_render_pass(Some(value), None) };
	}

	pub fn create_framebuffer(
		&self,
		info: backend::vk::FramebufferCreateInfo,
	) -> utility::Result<backend::vk::Framebuffer> {
		utility::as_vulkan_error(unsafe { self._internal.create_framebuffer(&info, None, None) })
	}

	pub fn destroy_framebuffer(&self, value: backend::vk::Framebuffer) {
		unsafe { self._internal.destroy_framebuffer(Some(value), None) };
	}

	pub fn create_command_pool(
		device: &Rc<Self>,
		queue_family_index: u32,
	) -> utility::Result<backend::vk::CommandPool> {
		let info = backend::vk::CommandPoolCreateInfoBuilder::new()
			.queue_family_index(queue_family_index)
			.build();
		utility::as_vulkan_error(unsafe { device._internal.create_command_pool(&info, None, None) })
	}

	pub fn destroy_command_pool(&self, value: backend::vk::CommandPool) {
		unsafe { self._internal.destroy_command_pool(Some(value), None) };
	}

	pub fn begin_command_buffer(&self, buffer: &command::Buffer) -> utility::Result<()> {
		let info = backend::vk::CommandBufferBeginInfoBuilder::new().build();
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
		info: backend::vk::RenderPassBeginInfo,
	) {
		unsafe {
			self._internal.cmd_begin_render_pass(
				*buffer.unwrap(),
				&info,
				backend::vk::SubpassContents::INLINE,
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

	pub fn destroy_fence(&self, value: backend::vk::Fence) {
		unsafe { self._internal.destroy_fence(Some(value), None) };
	}

	pub fn destroy_semaphore(&self, value: backend::vk::Semaphore) {
		unsafe { self._internal.destroy_semaphore(Some(value), None) };
	}
}
