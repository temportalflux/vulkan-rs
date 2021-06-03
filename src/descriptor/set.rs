use crate::{backend, descriptor};
use std::sync::Arc;

pub struct Set {
	internal: backend::vk::DescriptorSet,
	_layout: Arc<descriptor::SetLayout>,
}

impl Set {
	pub(crate) fn from(
		layout: Arc<descriptor::SetLayout>,
		internal: backend::vk::DescriptorSet,
	) -> Set {
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
