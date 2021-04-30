use crate::{
	backend, buffer, descriptor, device::logical, flags, image_view, utility::VulkanObject,
};
use std::rc::{Rc, Weak};

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
	pub set: Weak<descriptor::Set>,
	pub binding_index: u32,
	pub array_element: u32,
}

pub struct WriteOp {
	pub destination: UpdateOperationSet,
	pub kind: flags::DescriptorKind,
	pub objects: ObjectKind,
}

pub enum ObjectKind {
	Image(Vec<ImageKind>),
	Buffer(Vec<BufferKind>),
}

pub struct ImageKind {
	pub sampler: Rc<u8>,
	pub view: Rc<image_view::View>,
	pub layout: flags::ImageLayout,
}

pub struct BufferKind {
	pub buffer: Rc<buffer::Buffer>,
	pub offset: u64,
	pub range: u64,
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
		for (operation, (image_info, buffer_info)) in self.operations.iter().zip(
			write_images_per_operation
				.iter_mut()
				.zip(write_buffers_per_operation.iter_mut()),
		) {
			match operation {
				UpdateOperation::Write(op) => {
					match op.destination.set.upgrade() {
						Some(set_rc) => {
							let mut builder = backend::vk::WriteDescriptorSet::builder()
								.dst_set(*set_rc.unwrap())
								.dst_binding(op.destination.binding_index)
								.dst_array_element(op.destination.array_element)
								.descriptor_type(op.kind);
							match &op.objects {
								ObjectKind::Image(infos) => {
									for info in infos {
										image_info.push(
											backend::vk::DescriptorImageInfo::builder()
												// TODO: .sampler()
												.image_view(*info.view.unwrap())
												.image_layout(info.layout)
												.build(),
										);
									}
									builder = builder.image_info(&image_info[..]);
								}
								ObjectKind::Buffer(infos) => {
									for info in infos {
										buffer_info.push(
											backend::vk::DescriptorBufferInfo::builder()
												.buffer(*info.buffer.unwrap())
												.offset(info.offset)
												.range(info.range)
												.build(),
										);
									}
									builder = builder.buffer_info(&buffer_info[..]);
								}
							}
							writes.push(builder.build());
						}
						None => {}
					}
				}
				UpdateOperation::Copy(op) => {
					match (op.source.set.upgrade(), op.destination.set.upgrade()) {
						(Some(source_set), Some(destination_set)) => {
							copies.push(
								backend::vk::CopyDescriptorSet::builder()
									.src_set(*source_set.unwrap())
									.src_binding(op.source.binding_index)
									.src_array_element(op.source.array_element)
									.dst_set(*destination_set.unwrap())
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
			device
				.unwrap()
				.update_descriptor_sets(&writes[..], &copies[..]);
		}
	}
}
