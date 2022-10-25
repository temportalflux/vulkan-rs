use crate::{
	backend,
	descriptor::{self, layout::SetLayout, pool::Builder},
	device::logical,
	utility::{self, HandledObject},
};
use std::sync;

/// A descriptor pool is what creates [`Sets`](descriptor::Set) based on [`Layouts`](SetLayout).
///
/// They need to be allocated up-front with the total number of descriptor sets
/// and total number of individual kinds of descriptors.
///
/// If a pool is destroyed/dropped, all sets it created are also invalid
/// (and dropped if the user does not have shared ownership of the set).
pub struct Pool {
	owned_sets: Vec<sync::Arc<descriptor::Set>>,
	internal: backend::vk::DescriptorPool,
	device: sync::Arc<logical::Device>,
}

impl Pool {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub(crate) fn from(
		device: sync::Arc<logical::Device>,
		internal: backend::vk::DescriptorPool,
	) -> Pool {
		Pool {
			device,
			internal,
			owned_sets: Vec::new(),
		}
	}

	/// Allocates a set of descriptors based on a provided layout.
	///
	/// The returned vector of weak-reference-counted sets should not be upgraded
	/// into strong references until they need to be used.
	/// Since the pool owns these references, if the pool is dropped,
	/// the weak references will be invalidated unless something else is
	/// holding onto the reference for too long.
	pub fn allocate_descriptor_sets(
		&mut self,
		layouts: &Vec<sync::Arc<SetLayout>>,
	) -> utility::Result<Vec<sync::Weak<descriptor::Set>>> {
		self.allocate_named_descriptor_sets(
			&layouts
				.into_iter()
				.enumerate()
				.map(|(idx, layout)| (layout.clone(), format!("DescriptorSet.{}", idx)))
				.collect::<Vec<_>>(),
		)
	}

	/// Allocates a set of descriptors based on a provided layout.
	///
	/// The returned vector of weak-reference-counted sets should not be upgraded
	/// into strong references until they need to be used.
	/// Since the pool owns these references, if the pool is dropped,
	/// the weak references will be invalidated unless something else is
	/// holding onto the reference for too long.
	pub fn allocate_named_descriptor_sets(
		&mut self,
		layouts: &Vec<(sync::Arc<SetLayout>, String)>,
	) -> utility::Result<Vec<sync::Weak<descriptor::Set>>> {
		let set_layouts = layouts
			.iter()
			.map(|(layout, _)| ***layout)
			.collect::<Vec<_>>();
		let create_info = backend::vk::DescriptorSetAllocateInfo::builder()
			.descriptor_pool(**self)
			.set_layouts(&set_layouts)
			.build();
		let raw_sets = unsafe { self.device.allocate_descriptor_sets(&create_info) }?;
		Ok(raw_sets
			.into_iter()
			.enumerate()
			.map(|(idx, vk_desc_set)| {
				let (layout, name) = &layouts[idx];
				let set = descriptor::Set::from(layout.clone(), vk_desc_set);
				self.device
					.set_object_name_logged(&set.create_name(name.as_str()));
				let set = sync::Arc::new(set);
				self.owned_sets.push(set.clone());
				sync::Arc::downgrade(&set)
			})
			.collect())
	}
}

impl std::ops::Deref for Pool {
	type Target = backend::vk::DescriptorPool;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Drop for Pool {
	fn drop(&mut self) {
		unsafe {
			self.device.destroy_descriptor_pool(self.internal, None);
		}
	}
}

impl utility::HandledObject for Pool {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::DescriptorPool as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
