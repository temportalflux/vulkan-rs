use crate::{
	backend, buffer, command,
	device::logical,
	flags, image, pipeline, renderpass, structs,
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

	pub fn mark_pipeline_barrier(&self, barrier: command::PipelineBarrier) {
		use backend::version::DeviceV1_0;
		let mut memory_barriers: Vec<backend::vk::MemoryBarrier> = Vec::new();
		let mut buffer_barriers: Vec<backend::vk::BufferMemoryBarrier> = Vec::new();
		let mut image_barriers: Vec<backend::vk::ImageMemoryBarrier> = Vec::new();
		for barrier_kind in barrier.kinds {
			match barrier_kind {
				command::BarrierKind::Memory(info) => {
					memory_barriers.push(info.to_vk());
				}
				command::BarrierKind::Buffer(info) => {
					buffer_barriers.push(info.to_vk());
				}
				command::BarrierKind::Image(info) => {
					image_barriers.push(info.to_vk());
				}
			}
		}
		unsafe {
			self.device.unwrap().cmd_pipeline_barrier(
				self.internal,
				barrier.src_stage,
				barrier.dst_stage,
				backend::vk::DependencyFlags::empty(),
				&memory_barriers[..],
				&buffer_barriers[..],
				&image_barriers[..],
			)
		};
	}

	pub fn copy_buffer_to_image(
		&self,
		buffer: &buffer::Buffer,
		image: &image::Image,
		layout: flags::ImageLayout,
		regions: Vec<command::CopyBufferToImage>,
	) {
		use backend::version::DeviceV1_0;
		let regions = regions
			.into_iter()
			.map(|region| {
				backend::vk::BufferImageCopy::builder()
					.buffer_offset(region.buffer_offset as u64)
					.buffer_row_length(0)
					.buffer_image_height(0)
					.image_subresource(region.layers.to_vk())
					.image_offset(structs::Offset3D {
						x: region.offset.x(),
						y: region.offset.y(),
						z: region.offset.z(),
					})
					.image_extent(structs::Extent3D {
						width: region.size.x() as u32,
						height: region.size.y() as u32,
						depth: region.size.z() as u32,
					})
					.build()
			})
			.collect::<Vec<_>>();
		unsafe {
			self.device.unwrap().cmd_copy_buffer_to_image(
				self.internal,
				*buffer.unwrap(),
				*image.unwrap(),
				layout,
				&regions[..],
			);
		}
	}

	pub fn start_render_pass(
		&self,
		frame_buffer: &command::framebuffer::Framebuffer,
		render_pass: &renderpass::Pass,
		info: renderpass::RecordInstruction,
	) {
		use backend::version::DeviceV1_0;
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
		unsafe {
			self.device.unwrap().cmd_begin_render_pass(
				self.internal,
				&info,
				backend::vk::SubpassContents::INLINE,
			)
		};
	}

	pub fn stop_render_pass(&self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.unwrap().cmd_end_render_pass(self.internal) };
	}

	pub fn bind_pipeline(
		&self,
		pipeline: &pipeline::Pipeline,
		bind_point: flags::PipelineBindPoint,
	) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.unwrap()
				.cmd_bind_pipeline(self.internal, bind_point, *pipeline.unwrap())
		};
	}

	pub fn draw_vertices(&self, vertex_count: u32) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device.unwrap().cmd_draw(
				self.internal,
				vertex_count,
				/*instance count*/ 1,
				/*fist_index*/ 0,
				/*fist_instance*/ 0,
			)
		};
	}

	pub fn draw(
		&self,
		index_count: u32,
		first_index: u32,
		instance_count: u32,
		first_instance: u32,
		vertex_offset: i32,
	) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device.unwrap().cmd_draw_indexed(
				self.internal,
				index_count,
				instance_count,
				first_index,
				vertex_offset,
				first_instance,
			)
		};
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
