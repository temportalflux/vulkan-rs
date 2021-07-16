use crate::{backend, descriptor::layout::SetLayout, utility};
use std::sync::Arc;

/// A collection of descriptors as declared by a [`descriptor layout`](SetLayout).
/// A given set contains a number of bindings as described by the layout.
pub struct Set {
	internal: backend::vk::DescriptorSet,
	_layout: Arc<SetLayout>,
}

impl Set {
	pub(crate) fn from(layout: Arc<SetLayout>, internal: backend::vk::DescriptorSet) -> Set {
		Set {
			_layout: layout,
			internal,
		}
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
