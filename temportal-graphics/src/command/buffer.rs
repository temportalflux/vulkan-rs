use crate::{
	backend, buffer, command, descriptor,
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

	pub fn copy_buffer_to_buffer(
		&self,
		src: &buffer::Buffer,
		dst: &buffer::Buffer,
		regions: Vec<command::CopyBufferRange>,
	) {
		use backend::version::DeviceV1_0;
		let regions = regions
			.into_iter()
			.map(|region| {
				backend::vk::BufferCopy::builder()
					.src_offset(region.start_in_src as u64)
					.dst_offset(region.start_in_dst as u64)
					.size(region.size as u64)
					.build()
			})
			.collect::<Vec<_>>();
		unsafe {
			self.device.unwrap().cmd_copy_buffer(
				self.internal,
				*src.unwrap(),
				*dst.unwrap(),
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

	pub fn bind_descriptors(
		&self,
		bind_point: flags::PipelineBindPoint,
		layout: &pipeline::Layout,
		first_set_index: usize,
		sets: Vec<&descriptor::Set>,
	) {
		use backend::version::DeviceV1_0;
		let vk_sets = sets.iter().map(|set| *set.unwrap()).collect::<Vec<_>>();
		let offsets = Vec::new();
		unsafe {
			self.device.unwrap().cmd_bind_descriptor_sets(
				self.internal,
				bind_point,
				*layout.unwrap(),
				first_set_index as u32,
				&vk_sets[..],
				&offsets[..],
			)
		};
	}

	pub fn bind_vertex_buffers(
		&self,
		binding_index: u32,
		buffers: Vec<&buffer::Buffer>,
		offsets: Vec<u64>,
	) {
		use backend::version::DeviceV1_0;
		let vk_buffers = buffers
			.iter()
			.map(|buffer| *buffer.unwrap())
			.collect::<Vec<_>>();
		unsafe {
			self.device.unwrap().cmd_bind_vertex_buffers(
				self.internal,
				binding_index,
				&vk_buffers[..],
				&offsets[..],
			)
		};
	}

	pub fn bind_index_buffer(&self, buffer: &buffer::Buffer, offset: u64) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device.unwrap().cmd_bind_index_buffer(
				self.internal,
				*buffer.unwrap(),
				offset,
				backend::vk::IndexType::UINT32,
			)
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
		index_count: usize,
		first_index: usize,
		instance_count: usize,
		first_instance: usize,
		vertex_offset: usize,
	) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device.unwrap().cmd_draw_indexed(
				self.internal,
				index_count as u32,
				instance_count as u32,
				first_index as u32,
				vertex_offset as i32,
				first_instance as u32,
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
