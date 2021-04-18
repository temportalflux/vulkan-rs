use crate::{
	command,
	device::logical,
	flags, pipeline, renderpass,
	utility::{self, VulkanInfo, VulkanObject},
};
use erupt;

pub struct Buffer {
	_internal: erupt::vk::CommandBuffer,
}

impl Buffer {
	pub fn from(_internal: erupt::vk::CommandBuffer) -> Buffer {
		Buffer { _internal }
	}

	pub fn begin(&self, device: &logical::Device) -> utility::Result<()> {
		device.begin_command_buffer(&self)
	}

	pub fn end(&self, device: &logical::Device) -> utility::Result<()> {
		device.end_command_buffer(&self)
	}

	pub fn start_render_pass(
		&self,
		device: &logical::Device,
		frame_buffer: &command::framebuffer::Framebuffer,
		render_pass: &renderpass::Pass,
		info: renderpass::RecordInstruction,
	) {
		let clear_values = info
			.clear_values
			.iter()
			.map(|value| value.to_vk())
			.collect::<Vec<_>>();
		let info = erupt::vk::RenderPassBeginInfoBuilder::new()
			.render_pass(*render_pass.unwrap())
			.framebuffer(*frame_buffer.unwrap())
			.render_area(info.render_area)
			.clear_values(&clear_values)
			.build();
		device.begin_render_pass(&self, info);
	}

	pub fn stop_render_pass(&self, device: &logical::Device) {
		device.end_render_pass(&self);
	}

	pub fn bind_pipeline(
		&self,
		device: &logical::Device,
		pipeline: &pipeline::Pipeline,
		bind_point: flags::PipelineBindPoint,
	) {
		device.bind_pipeline(&self, &pipeline, bind_point);
	}

	pub fn draw(
		&self,
		device: &logical::Device,
		index_count: u32,
		first_index: u32,
		instance_count: u32,
		first_instance: u32,
		vertex_offset: i32,
	) {
		device.draw_indexed(
			&self,
			index_count,
			first_index,
			instance_count,
			first_instance,
			vertex_offset,
		);
	}
}

/// A trait exposing the internal value for the wrapped [`erupt::vk::CommandBuffer`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<erupt::vk::CommandBuffer> for Buffer {
	fn unwrap(&self) -> &erupt::vk::CommandBuffer {
		&self._internal
	}
	fn unwrap_mut(&mut self) -> &mut erupt::vk::CommandBuffer {
		&mut self._internal
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		// NO:OP - these are destroyed implicitly when command::Pool is destroyed
	}
}
