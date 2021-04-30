use crate::{
	backend, command,
	device::{logical, swapchain::Swapchain},
	flags, image, instance, pipeline,
	utility::{self, VulkanInfo, VulkanObject},
};
use backend::version::DeviceV1_0;
use std::rc::Rc;

/// A wrapper for a [`Vulkan LogicalDevice`](backend::Device),
/// which can send logical commands to the hardware.
pub struct Device {
	swapchain: backend::extensions::khr::Swapchain,
	internal: backend::Device,
}

impl Device {
	/// The internal constructor. Users should use [`Info.create_object`](struct.Info.html#method.create_object) to create a vulkan instance.
	pub fn from(instance: &instance::Instance, internal: backend::Device) -> Device {
		Device {
			swapchain: backend::extensions::khr::Swapchain::new(instance.unwrap(), &internal),
			internal,
		}
	}

	pub fn get_queue(device: &Rc<Self>, queue_family_index: usize) -> logical::Queue {
		let vk = device.get_device_queue(queue_family_index as u32);
		logical::Queue::from(device.clone(), vk, queue_family_index)
	}

	pub fn create_semaphores(
		device: &Rc<Self>,
		count: usize,
	) -> utility::Result<Vec<command::Semaphore>> {
		let mut vec: Vec<command::Semaphore> = Vec::new();
		let info = backend::vk::SemaphoreCreateInfo::builder().build();
		for _ in 0..count {
			let vk_semaphore =
				utility::as_vulkan_error(unsafe { device.internal.create_semaphore(&info, None) })?;
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
		let info = backend::vk::FenceCreateInfo::builder().flags(state).build();
		for _ in 0..count {
			let vk_fence =
				utility::as_vulkan_error(unsafe { device.internal.create_fence(&info, None) })?;
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
			self.internal
				.wait_for_fences(&fences, wait_for_all, timeout)
		})
	}

	pub fn acquire_next_image(
		&self,
		swapchain: &Swapchain,
		timeout: u64,
		semaphore: Option<&command::Semaphore>,
		fence: Option<&command::Fence>,
	) -> utility::Result<(/*image index*/ u32, /*is suboptimal*/ bool)> {
		utility::as_vulkan_error(unsafe {
			self.swapchain.acquire_next_image(
				*swapchain.unwrap(),
				timeout,
				semaphore.map_or(backend::vk::Semaphore::null(), |obj| *obj.unwrap()),
				fence.map_or(backend::vk::Fence::null(), |obj| *obj.unwrap()),
			)
		})
	}

	pub fn reset_fences(&self, fences: &[&command::Fence]) -> utility::Result<()> {
		let fences = fences.iter().map(|f| *f.unwrap()).collect::<Vec<_>>();
		utility::as_vulkan_error(unsafe { self.internal.reset_fences(&fences[..]) })
	}

	pub fn submit(
		&self,
		queue: &logical::Queue,
		infos: Vec<command::SubmitInfo>,
		signal_fence_when_complete: Option<&command::Fence>,
	) -> utility::Result<()> {
		let infos = infos.iter().map(|info| info.to_vk()).collect::<Vec<_>>();
		utility::as_vulkan_error(unsafe {
			self.internal.queue_submit(
				*queue.unwrap(),
				&infos,
				signal_fence_when_complete.map_or(backend::vk::Fence::null(), |obj| *obj.unwrap()),
			)
		})
	}

	pub fn present(
		&self,
		queue: &logical::Queue,
		info: command::PresentInfo,
	) -> utility::Result</*suboptimal*/ bool> {
		let vk_info = info.to_vk();
		utility::as_vulkan_error(unsafe { self.swapchain.queue_present(*queue.unwrap(), &vk_info) })
	}

	pub fn wait_until_idle(&self) -> utility::Result<()> {
		utility::as_vulkan_error(unsafe { self.internal.device_wait_idle() })
	}
}

/// A trait exposing the internal value for the wrapped [`backend::Device`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<backend::Device> for Device {
	fn unwrap(&self) -> &backend::Device {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::Device {
		&mut self.internal
	}
}

impl Drop for Device {
	fn drop(&mut self) {
		unsafe {
			self.internal.destroy_device(None);
		}
	}
}

#[doc(hidden)]
impl image::Owner for Device {
	fn destroy(&self, obj: &image::Image, _: Option<&vk_mem::Allocation>) -> utility::Result<()> {
		unsafe { self.internal.destroy_image(*obj.unwrap(), None) };
		Ok(())
	}
}

#[doc(hidden)]
impl Device {
	pub fn get_device_queue(&self, queue_family_index: u32) -> backend::vk::Queue {
		unsafe {
			self.internal
				.get_device_queue(queue_family_index, /*queue index*/ 0)
		}
	}

	pub fn create_swapchain(
		&self,
		info: backend::vk::SwapchainCreateInfoKHR,
	) -> utility::Result<backend::vk::SwapchainKHR> {
		utility::as_vulkan_error(unsafe { self.swapchain.create_swapchain(&info, None) })
	}

	pub fn destroy_swapchain(&self, value: backend::vk::SwapchainKHR) {
		unsafe { self.swapchain.destroy_swapchain(value, None) };
	}

	pub fn get_swapchain_images(
		&self,
		swapchain: &backend::vk::SwapchainKHR,
	) -> utility::Result<Vec<backend::vk::Image>> {
		utility::as_vulkan_error(unsafe { self.swapchain.get_swapchain_images(*swapchain) })
	}

	pub fn create_image_view(
		&self,
		info: backend::vk::ImageViewCreateInfo,
	) -> utility::Result<backend::vk::ImageView> {
		utility::as_vulkan_error(unsafe { self.internal.create_image_view(&info, None) })
	}

	pub fn destroy_image_view(&self, value: backend::vk::ImageView) {
		unsafe { self.internal.destroy_image_view(value, None) };
	}

	pub fn create_shader_module(
		&self,
		info: backend::vk::ShaderModuleCreateInfo,
	) -> utility::Result<backend::vk::ShaderModule> {
		utility::as_vulkan_error(unsafe { self.internal.create_shader_module(&info, None) })
	}

	pub fn destroy_shader_module(&self, value: backend::vk::ShaderModule) {
		unsafe { self.internal.destroy_shader_module(value, None) };
	}

	pub fn create_pipeline_layout(
		&self,
		info: backend::vk::PipelineLayoutCreateInfo,
	) -> utility::Result<backend::vk::PipelineLayout> {
		utility::as_vulkan_error(unsafe { self.internal.create_pipeline_layout(&info, None) })
	}

	pub fn destroy_pipeline_layout(&self, value: backend::vk::PipelineLayout) {
		unsafe { self.internal.destroy_pipeline_layout(value, None) };
	}

	pub fn create_pipeline_cache(
		&self,
		info: backend::vk::PipelineCacheCreateInfo,
	) -> utility::Result<backend::vk::PipelineCache> {
		utility::as_vulkan_error(unsafe { self.internal.create_pipeline_cache(&info, None) })
	}

	pub fn destroy_pipeline_cache(&self, value: backend::vk::PipelineCache) {
		unsafe { self.internal.destroy_pipeline_cache(value, None) };
	}

	pub fn create_graphics_pipelines(
		&self,
		cache: backend::vk::PipelineCache,
		infos: &[backend::vk::GraphicsPipelineCreateInfo],
	) -> utility::Result<Vec<backend::vk::Pipeline>> {
		match unsafe { self.internal.create_graphics_pipelines(cache, infos, None) } {
			Ok(pipelines) => Ok(pipelines),
			Err((pipelines, vk_result)) => match vk_result {
				backend::vk::Result::SUCCESS => Ok(pipelines),
				_ => Err(utility::Error::VulkanError(vk_result)),
			},
		}
	}

	pub fn destroy_pipeline(&self, value: backend::vk::Pipeline) {
		unsafe { self.internal.destroy_pipeline(value, None) };
	}

	pub fn create_render_pass(
		&self,
		info: backend::vk::RenderPassCreateInfo,
	) -> utility::Result<backend::vk::RenderPass> {
		utility::as_vulkan_error(unsafe { self.internal.create_render_pass(&info, None) })
	}

	pub fn destroy_render_pass(&self, value: backend::vk::RenderPass) {
		unsafe { self.internal.destroy_render_pass(value, None) };
	}

	pub fn create_framebuffer(
		&self,
		info: backend::vk::FramebufferCreateInfo,
	) -> utility::Result<backend::vk::Framebuffer> {
		utility::as_vulkan_error(unsafe { self.internal.create_framebuffer(&info, None) })
	}

	pub fn destroy_framebuffer(&self, value: backend::vk::Framebuffer) {
		unsafe { self.internal.destroy_framebuffer(value, None) };
	}

	pub fn create_command_pool(
		device: &Rc<Self>,
		queue_family_index: u32,
	) -> utility::Result<backend::vk::CommandPool> {
		let info = backend::vk::CommandPoolCreateInfo::builder()
			.queue_family_index(queue_family_index)
			.build();
		utility::as_vulkan_error(unsafe { device.internal.create_command_pool(&info, None) })
	}

	pub fn destroy_command_pool(&self, value: backend::vk::CommandPool) {
		unsafe { self.internal.destroy_command_pool(value, None) };
	}

	pub fn begin_render_pass(
		&self,
		buffer: &command::Buffer,
		info: backend::vk::RenderPassBeginInfo,
	) {
		unsafe {
			self.internal.cmd_begin_render_pass(
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
			self.internal
				.cmd_bind_pipeline(*buffer.unwrap(), point, *pipeline.unwrap())
		};
	}

	pub fn end_render_pass(&self, buffer: &command::Buffer) {
		unsafe { self.internal.cmd_end_render_pass(*buffer.unwrap()) };
	}

	pub fn draw(&self, buffer: &command::Buffer, index_count: u32) {
		unsafe {
			self.internal.cmd_draw(
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
			self.internal.cmd_draw_indexed(
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
		unsafe { self.internal.destroy_fence(value, None) };
	}

	pub fn destroy_semaphore(&self, value: backend::vk::Semaphore) {
		unsafe { self.internal.destroy_semaphore(value, None) };
	}
}
