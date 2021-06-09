use crate::{backend, descriptor::pool::Pool, device::logical, flags::DescriptorKind, utility};
use std::sync;

/// Constructs a [`Pool`](Pool) with a maximum number of sets and quantity of descriptors that can be allocated.
///
/// Pools need to know the total/maximum number of sets are how many sets can be created by
/// [`allocate_descriptor_sets`](Pool::allocate_descriptor_sets) for the lifetime of the pool.
/// Since any given set can contain multiple bindings/descriptors of different types
/// ([`with_binding`](crate::descriptor::layout::Builder::with_binding)),
/// the pool also needs to know the total number of descriptors that can ever be allocated.
/// This amount of descriptors is _shared_ between all allocated sets,
/// so if you configure the pool with 3 sets and with 2 descriptors, you wont be able to allocate 3 sets:
///
/// ```ignore
/// let pool = Pool::builder()
/// 	.with_total_set_count(3)
/// 	.with_descriptor(DescriptorKind::COMBINED_IMAGE_SAMPLER, 2)
/// 	.build(&logical_device)?;
/// let set_layout = Arc::new(
/// 	SetLayout::builder()
/// 	.with_binding(0, DescriptorKind::COMBINED_IMAGE_SAMPLER, 1, ShaderKind::Fragment)
/// 	.build(&logical_device)?
/// );
/// let sets = pool.allocate_descriptor_sets(
/// 	vec![&set_layout, &set_layout, &set_layout]
/// )?;
/// // will fail because there each of the 3 sets needs a descriptor, but there are only 2 descriptors.
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct Builder {
	/// The maximum number of sets ever allowed to be allocated from the pool.
	max_sets: u32,
	descriptors: Vec<backend::vk::DescriptorPoolSize>,
}

impl Builder {
	/// Sets the maximum number of [`sets`](super::super::Set) that can be created from the pool.
	pub fn with_total_set_count(mut self, max_set_count: u32) -> Self {
		self.max_sets = max_set_count;
		self
	}

	/// Denotates that the pool should create `amount` number of descriptors of a certain kind.
	/// This descriptor amount is shared between all descriptor sets allocated from the pool.
	pub fn with_descriptor(mut self, kind: DescriptorKind, amount: u32) -> Self {
		self.descriptors.push(backend::vk::DescriptorPoolSize {
			ty: kind,
			descriptor_count: amount,
		});
		self
	}
}

impl Builder {
	/// Creates an [`Pool`] object, thereby consuming the info.
	pub fn build(self, device: &sync::Arc<logical::Device>) -> utility::Result<Pool> {
		use backend::version::DeviceV1_0;
		let create_info = backend::vk::DescriptorPoolCreateInfo::builder()
			.max_sets(self.max_sets)
			.pool_sizes(&self.descriptors)
			.build();
		let internal = unsafe { device.create_descriptor_pool(&create_info, None) }?;
		Ok(Pool::from(device.clone(), internal))
	}
}
