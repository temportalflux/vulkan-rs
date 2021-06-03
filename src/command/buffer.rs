use crate::{
	backend, buffer, command, descriptor, device::logical, flags, image, pipeline, renderpass,
	utility,
};
use std::sync;

/// A ordered set of commands that will be executed on the GPU when they are submitted.
/// You can get a command buffer from [`command::Pool::allocate_buffers`](command::Pool::allocate_buffers).
pub struct Buffer {
	recording_framebuffer: Option<backend::vk::Framebuffer>,
	recording_render_pass: Option<backend::vk::RenderPass>,
	internal: backend::vk::CommandBuffer,
	device: sync::Arc<logical::Device>,
}

/// Internal only
impl Buffer {
	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::CommandBuffer,
	) -> Buffer {
		Buffer {
			device,
			internal,
			recording_render_pass: None,
			recording_framebuffer: None,
		}
	}
}

/// General operations used for every command buffer.
impl Buffer {
	/// Initializes the command buffer for writing commands.
	///
	/// Must be called before all other methods.
	///
	/// Equivalent to [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBeginCommandBuffer.html).
	pub fn begin(
		&self,
		usage: Option<flags::CommandBufferUsage>,
		primary: Option<&command::Buffer>,
	) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		let inheritance_info = match primary {
			Some(primary_buffer) => backend::vk::CommandBufferInheritanceInfo::builder()
				.render_pass(
					primary_buffer
						.recording_render_pass
						.unwrap_or(backend::vk::RenderPass::null()),
				)
				.subpass(0)
				.framebuffer(
					primary_buffer
						.recording_framebuffer
						.unwrap_or(backend::vk::Framebuffer::null()),
				)
				.occlusion_query_enable(false)
				.query_flags(backend::vk::QueryControlFlags::default())
				.pipeline_statistics(backend::vk::QueryPipelineStatisticFlags::default()),
			None => backend::vk::CommandBufferInheritanceInfo::builder(),
		};
		let info = backend::vk::CommandBufferBeginInfo::builder()
			.flags(usage.unwrap_or(flags::CommandBufferUsage::empty()))
			.inheritance_info(&inheritance_info);
		Ok(unsafe { self.device.begin_command_buffer(self.internal, &info) }?)
	}

	/// Finalizes the commands in the buffer.
	///
	/// Can only be called after [`begin`](Buffer::begin).
	///
	/// Equivalent to [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEndCommandBuffer.html).
	pub fn end(&self) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		Ok(unsafe { self.device.end_command_buffer(self.internal) }?)
	}

	/// Executes the commands within secondary command buffers.
	///
	/// Can only be called after [`begin`](Buffer::begin) and before [`end`](Buffer::end).
	///
	/// Equivalent to [`vkCmdExecuteCommands`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdExecuteCommands.html).
	pub fn execute(&self, secondary_buffers: Vec<&command::Buffer>) {
		use backend::version::DeviceV1_0;
		let unwraped = secondary_buffers
			.iter()
			.map(|cmd_buffer| ***cmd_buffer)
			.collect::<Vec<_>>();
		unsafe { self.device.cmd_execute_commands(self.internal, &unwraped) };
	}
}

/// Copying data operations!
impl Buffer {
	/// Adds a pipeline barrier to the buffer.
	/// See [`Pipeline Barrier`](command::barrier::Pipeline) for the kinds of barriers available.
	/// This can be used, for example, to move an image from one layout to another layout.
	///
	/// Can only be called after [`begin`](Buffer::begin) and before [`end`](Buffer::end).
	///
	/// Equivalent to [`vkCmdPipelineBarrier`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPipelineBarrier.html).
	pub fn mark_pipeline_barrier(&self, barrier: command::barrier::Pipeline) {
		use backend::version::DeviceV1_0;
		use command::barrier::Kind::*;
		let mut memory_barriers: Vec<backend::vk::MemoryBarrier> = Vec::new();
		let mut buffer_barriers: Vec<backend::vk::BufferMemoryBarrier> = Vec::new();
		let mut image_barriers: Vec<backend::vk::ImageMemoryBarrier> = Vec::new();
		for barrier_kind in barrier.kinds {
			match barrier_kind {
				Memory(info) => {
					memory_barriers.push(info.into());
				}
				Buffer(info) => {
					buffer_barriers.push(info.as_vk());
				}
				Image(info) => {
					image_barriers.push(info.as_vk());
				}
			}
		}
		unsafe {
			self.device.cmd_pipeline_barrier(
				self.internal,
				barrier.src_stage.into(),
				barrier.dst_stage.into(),
				backend::vk::DependencyFlags::empty(),
				&memory_barriers[..],
				&buffer_barriers[..],
				&image_barriers[..],
			)
		};
	}

	/// Copies data from some buffer to an image for a set region of the buffer.
	///
	/// Can only be called after [`begin`](Buffer::begin) and before [`end`](Buffer::end).
	///
	/// Equivalent to [`vkCmdCopyBufferToImage`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage.html).
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
					.image_subresource(region.layers.into())
					.image_offset(region.offset)
					.image_extent(region.size)
					.build()
			})
			.collect::<Vec<_>>();
		unsafe {
			self.device.cmd_copy_buffer_to_image(
				self.internal,
				**buffer,
				**image,
				layout.into(),
				&regions[..],
			);
		}
	}

	/// Copies some data from one buffer to another.
	///
	/// Can only be called after [`begin`](Buffer::begin) and before [`end`](Buffer::end).
	///
	/// Equivalent to [`vkCmdCopyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBuffer.html).
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
			self.device
				.cmd_copy_buffer(self.internal, **src, **dst, &regions[..]);
		}
	}
}

/// Render Pass operations
impl Buffer {
	/// Starts the render pass for recording rendering instructions.
	///
	/// Can only be called after [`begin`](Buffer::begin) and before [`end`](Buffer::end).
	///
	/// Equivalent to [`vkCmdBeginRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass.html).
	pub fn start_render_pass(
		&mut self,
		frame_buffer: &command::framebuffer::Framebuffer,
		render_pass: &renderpass::Pass,
		info: renderpass::RecordInstruction,
		uses_secondary_buffers: bool,
	) {
		use backend::version::DeviceV1_0;
		let clear_values = info
			.clear_values
			.iter()
			.map(|value| (*value).into())
			.collect::<Vec<_>>();
		let info = backend::vk::RenderPassBeginInfo::builder()
			.render_pass(**render_pass)
			.framebuffer(**frame_buffer)
			.render_area(info.render_area)
			.clear_values(&clear_values)
			.build();
		unsafe {
			self.device.cmd_begin_render_pass(
				self.internal,
				&info,
				if uses_secondary_buffers {
					backend::vk::SubpassContents::SECONDARY_COMMAND_BUFFERS
				} else {
					backend::vk::SubpassContents::INLINE
				},
			)
		};
		self.recording_render_pass = Some(**render_pass);
		self.recording_framebuffer = Some(**frame_buffer);
	}

	/// Moves to the next subpass in the active render pass.
	///
	/// Can only be called after [`start_render_pass`](Buffer::start_render_pass) and before [`stop_render_pass`](Buffer::stop_render_pass).
	///
	/// Equivalent to [`vkCmdNextSubpass`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass.html).
	pub fn next_subpass(&mut self, uses_secondary_buffers: bool) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device.cmd_next_subpass(
				self.internal,
				if uses_secondary_buffers {
					backend::vk::SubpassContents::SECONDARY_COMMAND_BUFFERS
				} else {
					backend::vk::SubpassContents::INLINE
				},
			)
		};
	}

	/// Stops the render pass for recording rendering instructions.
	///
	/// Can only be called after [`start_render_pass`](Buffer::start_render_pass) and before [`end`](Buffer::end).
	///
	/// Equivalent to [`vkCmdEndRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass.html).
	pub fn stop_render_pass(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.cmd_end_render_pass(self.internal) };
		self.recording_render_pass = None;
		self.recording_framebuffer = None;
	}
}

/// Pipeline operations
impl Buffer {
	/// Binds a pipeline to the buffer. The pipeline will stay bound until another pipeline is bound.
	///
	/// Can only be called after [`start_render_pass`](Buffer::start_render_pass) and before [`stop_render_pass`](Buffer::stop_render_pass).
	///
	/// Equivalent to [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindPipeline.html).
	pub fn bind_pipeline(
		&self,
		pipeline: &pipeline::Pipeline,
		bind_point: flags::PipelineBindPoint,
	) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device
				.cmd_bind_pipeline(self.internal, bind_point, **pipeline)
		};
	}

	/// Binds descriptors for a given pipeline layout.
	/// The descriptors will stay bound until another pipeline is bound via [`bind_pipeline`](Buffer::bind_pipeline).
	///
	/// Can only be called after [`start_render_pass`](Buffer::start_render_pass) and before [`stop_render_pass`](Buffer::stop_render_pass).
	///
	/// If used, this should only be called after [`bind_pipeline`](Buffer::bind_pipeline).
	///
	/// Equivalent to [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindDescriptorSets.html).
	pub fn bind_descriptors(
		&self,
		bind_point: flags::PipelineBindPoint,
		layout: &pipeline::Layout,
		first_set_index: usize,
		sets: Vec<&descriptor::Set>,
	) {
		use backend::version::DeviceV1_0;
		let vk_sets = sets.iter().map(|set| ***set).collect::<Vec<_>>();
		let offsets = Vec::new();
		unsafe {
			self.device.cmd_bind_descriptor_sets(
				self.internal,
				bind_point,
				**layout,
				first_set_index as u32,
				&vk_sets[..],
				&offsets[..],
			)
		};
	}

	/// Sets the scissor that should be used for pipelines which use the [`DynamicState.SCISSOR`](flags::DynamicState::SCISSOR) flag.
	///
	/// Can only be called after [`start_render_pass`](Buffer::start_render_pass) and before [`stop_render_pass`](Buffer::stop_render_pass).
	///
	/// Equivalent to [`vkCmdSetScissor`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissor.html).
	pub fn set_dynamic_scissors(&self, scissors: Vec<utility::Scissor>) {
		use backend::version::DeviceV1_0;
		let scissors = scissors
			.into_iter()
			.map(|scissor| scissor.into())
			.collect::<Vec<_>>();
		unsafe { self.device.cmd_set_scissor(self.internal, 0, &scissors[..]) };
	}
}

// Data Buffer operations
impl Buffer {
	/// Binds buffers which contain vertex/instance data.
	///
	/// Can only be called after [`start_render_pass`](Buffer::start_render_pass) and before [`stop_render_pass`](Buffer::stop_render_pass).
	///
	/// If used, this should only be called after [`bind_pipeline`](Buffer::bind_pipeline).
	///
	/// Equivalent to [`vkCmdBindVertexBuffers`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers.html).
	pub fn bind_vertex_buffers(
		&self,
		binding_index: u32,
		buffers: Vec<&buffer::Buffer>,
		offsets: Vec<u64>,
	) {
		use backend::version::DeviceV1_0;
		let vk_buffers = buffers.iter().map(|buffer| ***buffer).collect::<Vec<_>>();
		unsafe {
			self.device.cmd_bind_vertex_buffers(
				self.internal,
				binding_index,
				&vk_buffers[..],
				&offsets[..],
			)
		};
	}

	/// Binds buffers which contain index data for vertex buffers.
	///
	/// Can only be called after [`start_render_pass`](Buffer::start_render_pass) and before [`stop_render_pass`](Buffer::stop_render_pass).
	///
	/// If used, this should only be called after [`bind_pipeline`](Buffer::bind_pipeline).
	///
	/// Equivalent to [`vkCmdBindIndexBuffer`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindIndexBuffer.html).
	pub fn bind_index_buffer(&self, buffer: &buffer::Buffer, offset: u64) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.device.cmd_bind_index_buffer(
				self.internal,
				**buffer,
				offset,
				backend::vk::IndexType::UINT32,
			)
		};
	}

	/// Draws data from the currently bound vertex and index buffers for the current pipeline + descriptors.
	///
	/// Should be called after:
	/// - [`bind_pipeline`](Buffer::bind_pipeline)
	/// - [`bind_descriptors`](Buffer::bind_descriptors)
	/// - [`bind_vertex_buffers`](Buffer::bind_vertex_buffers)
	/// - [`bind_index_buffer`](Buffer::bind_index_buffer)
	///
	/// Can only be called before [`stop_render_pass`](Buffer::stop_render_pass).
	///
	/// Equivalent to [`vkCmdDrawIndexed`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexed.html).
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
			self.device.cmd_draw_indexed(
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

impl std::ops::Deref for Buffer {
	type Target = backend::vk::CommandBuffer;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		// NO:OP - these are destroyed implicitly when command::Pool is destroyed
	}
}
