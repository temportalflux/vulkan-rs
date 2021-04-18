use crate::{
	command,
	device::{logical, swapchain::Swapchain},
	flags, pipeline,
	utility::{self, VulkanInfo, VulkanObject},
};
use erupt;
use std::rc::Rc;

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

	pub fn get_queue(&self, queue_family_index: u32) -> logical::Queue {
		logical::Queue::from(self.get_device_queue(queue_family_index))
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

	pub fn create_semaphores(
		device: &Rc<Self>,
		count: usize,
	) -> utility::Result<Vec<command::Semaphore>> {
		let mut vec: Vec<command::Semaphore> = Vec::new();
		let info = erupt::vk::SemaphoreCreateInfoBuilder::new().build();
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
		let info = erupt::vk::FenceCreateInfoBuilder::new()
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
			.into_iter()
			.map(|info| info.to_vk())
			.collect::<Vec<_>>();
		utility::as_vulkan_error(unsafe {
			self._internal.queue_submit(
				*queue.unwrap(),
				crate::into_builders!(infos),
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

impl Drop for Device {
	fn drop(&mut self) {
		unsafe {
			self._internal.destroy_device(None);
		}
	}
}

#[doc(hidden)]
impl Device {
	pub fn get_device_queue(&self, queue_family_index: u32) -> erupt::vk::Queue {
		unsafe {
			self._internal
				.get_device_queue(queue_family_index, /*queue index*/ 0, None)
		}
	}

	pub fn create_swapchain(
		&self,
		info: erupt::vk::SwapchainCreateInfoKHR,
	) -> utility::Result<erupt::vk::SwapchainKHR> {
		utility::as_vulkan_error(unsafe { self._internal.create_swapchain_khr(&info, None, None) })
	}

	pub fn destroy_swapchain(&self, value: erupt::vk::SwapchainKHR) {
		unsafe { self._internal.destroy_swapchain_khr(Some(value), None) };
	}

	pub fn get_swapchain_images(
		&self,
		swapchain: &erupt::vk::SwapchainKHR,
	) -> utility::Result<Vec<erupt::vk::Image>> {
		utility::as_vulkan_error(unsafe {
			self._internal.get_swapchain_images_khr(*swapchain, None)
		})
	}

	pub fn destroy_image(&self, value: erupt::vk::Image) {
		unsafe { self._internal.destroy_image(Some(value), None) };
	}

	pub fn create_image_view(
		&self,
		info: erupt::vk::ImageViewCreateInfo,
	) -> utility::Result<erupt::vk::ImageView> {
		utility::as_vulkan_error(unsafe { self._internal.create_image_view(&info, None, None) })
	}

	pub fn destroy_image_view(&self, value: erupt::vk::ImageView) {
		unsafe { self._internal.destroy_image_view(Some(value), None) };
	}

	pub fn create_shader_module(
		&self,
		info: erupt::vk::ShaderModuleCreateInfo,
	) -> utility::Result<erupt::vk::ShaderModule> {
		utility::as_vulkan_error(unsafe { self._internal.create_shader_module(&info, None, None) })
	}

	pub fn destroy_shader_module(&self, value: erupt::vk::ShaderModule) {
		unsafe { self._internal.destroy_shader_module(Some(value), None) };
	}

	pub fn create_pipeline_layout(
		&self,
		info: erupt::vk::PipelineLayoutCreateInfo,
	) -> utility::Result<erupt::vk::PipelineLayout> {
		utility::as_vulkan_error(unsafe {
			self._internal.create_pipeline_layout(&info, None, None)
		})
	}

	pub fn destroy_pipeline_layout(&self, value: erupt::vk::PipelineLayout) {
		unsafe { self._internal.destroy_pipeline_layout(Some(value), None) };
	}

	pub fn create_graphics_pipelines(
		&self,
		infos: &[erupt::vk::GraphicsPipelineCreateInfoBuilder<'_>],
	) -> utility::Result<Vec<erupt::vk::Pipeline>> {
		utility::as_vulkan_error(unsafe {
			self._internal.create_graphics_pipelines(None, infos, None)
		})
	}

	pub fn destroy_pipeline(&self, value: erupt::vk::Pipeline) {
		unsafe { self._internal.destroy_pipeline(Some(value), None) };
	}

	pub fn create_render_pass(
		&self,
		info: erupt::vk::RenderPassCreateInfo,
	) -> utility::Result<erupt::vk::RenderPass> {
		utility::as_vulkan_error(unsafe { self._internal.create_render_pass(&info, None, None) })
	}

	pub fn destroy_render_pass(&self, value: erupt::vk::RenderPass) {
		unsafe { self._internal.destroy_render_pass(Some(value), None) };
	}

	pub fn create_framebuffer(
		&self,
		info: erupt::vk::FramebufferCreateInfo,
	) -> utility::Result<erupt::vk::Framebuffer> {
		utility::as_vulkan_error(unsafe { self._internal.create_framebuffer(&info, None, None) })
	}

	pub fn destroy_framebuffer(&self, value: erupt::vk::Framebuffer) {
		unsafe { self._internal.destroy_framebuffer(Some(value), None) };
	}

	pub fn create_command_pool(
		device: &Rc<Self>,
		queue_family_index: u32,
	) -> utility::Result<erupt::vk::CommandPool> {
		let info = erupt::vk::CommandPoolCreateInfoBuilder::new()
			.queue_family_index(queue_family_index)
			.build();
		utility::as_vulkan_error(unsafe { device._internal.create_command_pool(&info, None, None) })
	}

	pub fn destroy_command_pool(&self, value: erupt::vk::CommandPool) {
		unsafe { self._internal.destroy_command_pool(Some(value), None) };
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

	pub fn destroy_fence(&self, value: erupt::vk::Fence) {
		unsafe { self._internal.destroy_fence(Some(value), None) };
	}

	pub fn destroy_semaphore(&self, value: erupt::vk::Semaphore) {
		unsafe { self._internal.destroy_semaphore(Some(value), None) };
	}
}
