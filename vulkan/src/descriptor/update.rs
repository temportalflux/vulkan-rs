use crate::{backend, buffer, descriptor, device::logical, flags, image_view, sampler};
use std::sync;

pub struct SetUpdate {
	operations: Vec<UpdateOperation>,
}

impl Default for SetUpdate {
	fn default() -> SetUpdate {
		SetUpdate {
			operations: Vec::new(),
		}
	}
}

pub enum UpdateOperation {
	Write(WriteOp),
	Copy(CopyOp),
}

pub struct UpdateOperationSet {
	pub set: sync::Weak<descriptor::Set>,
	pub binding_index: u32,
	pub array_element: u32,
}

pub struct WriteOp {
	pub destination: UpdateOperationSet,
	pub kind: flags::DescriptorKind,
	pub object: ObjectKind,
}

pub enum ObjectKind {
	Image(Vec<ImageKind>),
	Buffer(Vec<BufferKind>),
}

pub struct ImageKind {
	pub sampler: sync::Arc<sampler::Sampler>,
	pub view: sync::Arc<image_view::View>,
	pub layout: flags::ImageLayout,
}

pub struct BufferKind {
	pub buffer: sync::Arc<buffer::Buffer>,
	pub offset: usize,
	pub range: usize,
}

pub struct CopyOp {
	pub source: UpdateOperationSet,
	pub destination: UpdateOperationSet,
	pub descriptor_count: u32,
}

impl SetUpdate {
	pub fn with(mut self, operation: UpdateOperation) -> Self {
		self.operations.push(operation);
		self
	}

	pub fn apply(self, device: &logical::Device) {
		use backend::version::DeviceV1_0;
		let mut write_images_per_operation: Vec<Vec<backend::vk::DescriptorImageInfo>> =
			Vec::with_capacity(self.operations.len());
		let mut write_buffers_per_operation: Vec<Vec<backend::vk::DescriptorBufferInfo>> =
			Vec::with_capacity(self.operations.len());
		let mut writes = Vec::new();
		let mut copies = Vec::new();
		for (idx, operation) in self.operations.iter().enumerate() {
			match operation {
				UpdateOperation::Write(op) => {
					if let Some(set_rc) = op.destination.set.upgrade() {
						let mut builder = backend::vk::WriteDescriptorSet::builder()
							.dst_set(**set_rc)
							.dst_binding(op.destination.binding_index)
							.dst_array_element(op.destination.array_element)
							.descriptor_type(op.kind);
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
								}
								builder =
									builder.buffer_info(&write_buffers_per_operation[idx_ops][..]);
							}
						}
						writes.push(builder.build());
					} else {
						log::error!("Encounted invalid descriptor set for write operate, will skip operation {}", idx);
					}
				}
				UpdateOperation::Copy(op) => {
					match (op.source.set.upgrade(), op.destination.set.upgrade()) {
						(Some(source_set), Some(destination_set)) => {
							copies.push(
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
						}
						_ => {}
					}
				}
			}
		}
		unsafe {
			device.update_descriptor_sets(&writes[..], &copies[..]);
		}
	}
}
