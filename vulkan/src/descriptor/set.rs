use crate::{
	backend,
	descriptor::layout::SetLayout,
	utility::{self, BoundObject},
};
use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
};

/// A collection of descriptors as declared by a [`descriptor layout`](SetLayout).
/// A given set contains a number of bindings as described by the layout.
pub struct Set {
	internal: backend::vk::DescriptorSet,
	bound_objects: Mutex<HashMap<(u32, u32), Vec<BoundObject>>>,
	_layout: Arc<SetLayout>,
}

impl Set {
	pub(crate) fn from(layout: Arc<SetLayout>, internal: backend::vk::DescriptorSet) -> Set {
		Set {
			_layout: layout,
			bound_objects: Mutex::new(HashMap::new()),
			internal,
		}
	}

	pub(crate) fn set_bound(&self, idx: (u32, u32), objects: Vec<BoundObject>) {
		self.bound_objects.lock().unwrap().insert(idx, objects);
	}

	pub(crate) fn get_bound(&self, idx: (u32, u32)) -> Vec<BoundObject> {
		self.bound_objects
			.lock()
			.unwrap()
			.get(&idx)
			.unwrap()
			.clone()
	}

	pub(crate) fn get_all_bound(&self) -> Vec<BoundObject> {
		self.bound_objects
			.lock()
			.unwrap()
			.iter()
			.fold(Vec::new(), |mut vec, (_idx, objects)| {
				vec.append(&mut objects.clone());
				vec
			})
	}
}

impl std::ops::Deref for Set {
	type Target = backend::vk::DescriptorSet;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl utility::HandledObject for Set {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::DescriptorSet as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.as_raw()
	}
}
