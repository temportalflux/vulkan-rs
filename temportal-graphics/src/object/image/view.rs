use erupt;

/// A wrapper around [`Image View`](erupt::vk::ImageView).
pub struct View {
	_internal: erupt::vk::ImageView,
}

impl View {
	pub fn from(_internal: erupt::vk::ImageView) -> View {
		View { _internal }
	}
}
