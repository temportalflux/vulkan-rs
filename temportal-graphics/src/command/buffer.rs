use crate::{
	backend, command,
	device::logical,
	flags, pipeline, renderpass,
	utility::{self, VulkanInfo, VulkanObject},
};
use std::rc::Rc;

pub struct Buffer {
	device: Rc<logical::Device>,
	internal: backend::vk::CommandBuffer,
}

impl Buffer {
	pub fn from(device: Rc<logical::Device>, internal: backend::vk::CommandBuffer) -> Buffer {
		Buffer { device, internal }
	}

	pub fn begin(&self, usage: Option<flags::CommandBufferUsage>) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		let info = backend::vk::CommandBufferBeginInfo::builder()
			.flags(usage.unwrap_or(flags::CommandBufferUsage::empty()))
			.build();
		utility::as_vulkan_error(unsafe {
			self.device
				.unwrap()
				.begin_command_buffer(self.internal, &info)
		})
	}

	pub fn end(&self) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		utility::as_vulkan_error(unsafe { self.device.unwrap().end_command_buffer(self.internal) })
	}

	pub fn start_render_pass(
		&self,
		frame_buffer: &command::framebuffer::Framebuffer,
		render_pass: &renderpass::Pass,
		info: renderpass::RecordInstruction,
	) {
		let clear_values = info
			.clear_values
			.iter()
			.map(|value| value.to_vk())
			.collect::<Vec<_>>();
		let info = backend::vk::RenderPassBeginInfo::builder()
			.render_pass(*render_pass.unwrap())
			.framebuffer(*frame_buffer.unwrap())
			.render_area(info.render_area)
			.clear_values(&clear_values)
			.build();
		self.device.begin_render_pass(&self, info);
	}

	pub fn stop_render_pass(&self) {
		self.device.end_render_pass(&self);
	}

	pub fn bind_pipeline(
		&self,
		pipeline: &pipeline::Pipeline,
		bind_point: flags::PipelineBindPoint,
	) {
		self.device.bind_pipeline(&self, &pipeline, bind_point);
	}

	pub fn draw_vertices(&self, vertex_count: u32) {
		self.device.draw(&self, vertex_count);
	}

	pub fn draw(
		&self,
		index_count: u32,
		first_index: u32,
		instance_count: u32,
		first_instance: u32,
		vertex_offset: i32,
	) {
		self.device.draw_indexed(
			&self,
			index_count,
			first_index,
			instance_count,
			first_instance,
			vertex_offset,
		);
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::CommandBuffer`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::CommandBuffer> for Buffer {
	fn unwrap(&self) -> &backend::vk::CommandBuffer {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::CommandBuffer {
		&mut self.internal
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		// NO:OP - these are destroyed implicitly when command::Pool is destroyed
	}
}
