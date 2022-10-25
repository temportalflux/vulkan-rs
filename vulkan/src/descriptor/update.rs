//! Attaching data to descriptors.

use crate::{backend, buffer, descriptor, device::logical, flags, image_view, sampler};
use std::sync;

/// A collection of operations to perform on a descriptor set.
#[derive(Default, Clone)]
pub struct Queue {
	operations: Vec<Operation>,
}

/// A single operation to perform on a descriptor set.
#[derive(Clone)]
pub enum Operation {
	/// An operation which writes binding data to a descriptor.
	/// Equivalent to [`VkWriteDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSet.html).
	Write(WriteOp),
	/// An operation which copies binding data from one descriptor to another.
	Copy(CopyOp),
}

/// The descriptor that will be updated in a given set.
#[derive(Clone)]
pub struct Descriptor {
	pub set: sync::Weak<descriptor::Set>,
	pub binding_index: u32,
	pub array_element: u32,
}

/// The body of the [`Write`](Operation::Write) operation.
#[derive(Clone)]
pub struct WriteOp {
	pub destination: Descriptor,
	pub kind: flags::DescriptorKind,
	pub object: ObjectKind,
}

/// The kind of object that can be bound to a descriptor.
/// [`VkWriteDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSet.html)
/// allows for multiple descriptor writes in the same operation, which is why each enum value is an array of bound objects.
#[derive(Clone)]
pub enum ObjectKind {
	/// The descriptor will be bound to an [`Image View`](image_view::View).
	Image(Vec<ImageKind>),
	/// The descriptor will be bound to a [`Buffer`](buffer::Buffer).
	Buffer(Vec<BufferKind>),
}

/// Body for attaching an [`Image View`](image_view::View) & [`Sampler`](sampler::Sampler) to a descriptor binding.
#[derive(Clone)]
pub struct ImageKind {
	pub sampler: sync::Arc<sampler::Sampler>,
	pub view: sync::Arc<image_view::View>,
	pub layout: flags::ImageLayout,
}

/// Body for attaching a [`Buffer`](buffer::Buffer) to a descriptor binding.
#[derive(Clone)]
pub struct BufferKind {
	pub buffer: sync::Arc<buffer::Buffer>,
	/// The number of bytes from the start of the buffer that the descriptor should start reading at.
	pub offset: usize,
	/// The number of bytes starting at `offset` that the descriptor can read from.
	pub range: usize,
}

/// The body of the [`Copy`](Operation::Copy) operation.
#[derive(Clone)]
pub struct CopyOp {
	pub source: Descriptor,
	pub destination: Descriptor,
	pub descriptor_count: u32,
}

impl Queue {
	/// Enqueues an operation to the list to be executed when `apply` is called.
	pub fn with(mut self, operation: Operation) -> Self {
		self.operations.push(operation);
		self
	}

	/// Applies all enqueued operations, thereby consuming the queue.
	///
	/// While it is technically safe to copy or move the queue around between construction, enqueuing operations, and calling `apply`,
	/// it is recommended that it all be done in the same stack so that the bound objects aren't kept around longer than needed
	/// (because operations hold strong reference counted pointers to the objects it will bind).
	pub fn apply(self, device: &logical::Device) {
		let mut write_images_per_operation: Vec<Vec<backend::vk::DescriptorImageInfo>> =
			Vec::with_capacity(self.operations.len());
		let mut write_buffers_per_operation: Vec<Vec<backend::vk::DescriptorBufferInfo>> =
			Vec::with_capacity(self.operations.len());
		let mut vk_writes = Vec::new();
		let mut rc_writes = Vec::new();
		let mut vk_copies = Vec::new();
		let mut rc_copies = Vec::new();
		for (idx, operation) in self.operations.iter().enumerate() {
			match operation {
				Operation::Write(op) => {
					if let Some(set_rc) = op.destination.set.upgrade() {
						let mut builder = backend::vk::WriteDescriptorSet::builder()
							.dst_set(**set_rc)
							.dst_binding(op.destination.binding_index)
							.dst_array_element(op.destination.array_element)
							.descriptor_type(op.kind);
						let mut object_rcs: Vec<
							sync::Arc<dyn std::any::Any + 'static + Send + Sync>,
						> = Vec::new();
						match &op.object {
							ObjectKind::Image(infos) => {
								let idx_ops = write_images_per_operation.len();
								write_images_per_operation.push(Vec::new());
								for info in infos {
									write_images_per_operation[idx_ops].push(
										backend::vk::DescriptorImageInfo::builder()
											.sampler(**info.sampler)
											.image_view(**info.view)
											.image_layout(info.layout.into())
											.build(),
									);
									object_rcs.push(info.sampler.clone());
									object_rcs.push(info.view.clone());
								}
								builder =
									builder.image_info(&write_images_per_operation[idx_ops][..]);
							}
							ObjectKind::Buffer(infos) => {
								let idx_ops = write_buffers_per_operation.len();
								write_buffers_per_operation.push(Vec::new());
								for info in infos {
									write_buffers_per_operation[idx_ops].push(
										backend::vk::DescriptorBufferInfo::builder()
											.buffer(**info.buffer)
											.offset(info.offset as u64)
											.range(info.range as u64)
											.build(),
									);
									object_rcs.push(info.buffer.clone());
								}
								builder =
									builder.buffer_info(&write_buffers_per_operation[idx_ops][..]);
							}
						}
						vk_writes.push(builder.build());
						rc_writes.push((
							set_rc.clone(),
							(op.destination.binding_index, op.destination.array_element),
							object_rcs,
						));
					} else {
						log::error!("Encounted invalid descriptor set for write operate, will skip operation {}", idx);
					}
				}
				Operation::Copy(op) => {
					match (op.source.set.upgrade(), op.destination.set.upgrade()) {
						(Some(source_set), Some(destination_set)) => {
							vk_copies.push(
								backend::vk::CopyDescriptorSet::builder()
									.src_set(**source_set)
									.src_binding(op.source.binding_index)
									.src_array_element(op.source.array_element)
									.dst_set(**destination_set)
									.dst_binding(op.destination.binding_index)
									.dst_array_element(op.destination.array_element)
									.descriptor_count(op.descriptor_count)
									.build(),
							);
							let object_rcs = source_set
								.get_bound((op.source.binding_index, op.source.array_element));
							rc_copies.push((
								destination_set.clone(),
								(op.destination.binding_index, op.destination.array_element),
								object_rcs,
							));
						}
						_ => unimplemented!(),
					}
				}
			}
		}
		unsafe {
			device.update_descriptor_sets(&vk_writes[..], &vk_copies[..]);
		}
		for (arc_set, idx, rcs) in rc_writes.into_iter() {
			arc_set.set_bound(idx, rcs);
		}
		for (arc_set, idx, rcs) in rc_copies.into_iter() {
			arc_set.set_bound(idx, rcs);
		}
	}
}
