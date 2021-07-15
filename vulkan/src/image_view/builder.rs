use crate::{
	backend,
	device::logical,
	flags::{ComponentSwizzle, ImageViewType},
	image::Image,
	image_view::View,
	structs::{subresource, ComponentMapping},
	utility,
};

use std::sync;

/// The builder for [`View`] objects.
pub struct Builder {
	image: sync::Weak<Image>,
	view_type: ImageViewType,
	components: ComponentMapping,
	subresource_range: subresource::Range,
	name: Option<String>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			image: sync::Weak::new(),
			view_type: ImageViewType::TYPE_2D,
			components: ComponentMapping {
				r: ComponentSwizzle::R,
				g: ComponentSwizzle::G,
				b: ComponentSwizzle::B,
				a: ComponentSwizzle::A,
			},
			subresource_range: subresource::Range::default(),
			name: None,
		}
	}
}

impl Builder {
	/// Mutates the builder to set the image that the view wraps/owns.
	///
	/// The builder will not hold a strong reference to the image,
	/// so if the image is dropped between `for_image` and `build`,
	/// the builder will fail to create the [`View`].
	pub fn for_image(mut self, image: sync::Arc<Image>) -> Self {
		self.image = sync::Arc::downgrade(&image);
		self
	}

	/// Mutates the builder to use a specific [`view type`](ImageViewType).
	/// By default, the view type is [`2D`](ImageViewType::TYPE_2D).
	pub fn with_view_type(mut self, view_type: ImageViewType) -> Self {
		self.view_type = view_type;
		self
	}

	/// Mutates the builder to use a specific component mapping.
	/// By default, the value uses the default `RGBA` swizzle components.
	pub fn with_components(mut self, components: ComponentMapping) -> Self {
		self.components = components;
		self
	}

	/// Mutates the builder to use a specific mip/aspect/layers range.
	pub fn with_range(mut self, subresource_range: subresource::Range) -> Self {
		self.subresource_range = subresource_range;
		self
	}

}

impl utility::NameableBuilder for Builder {
	fn with_optname(mut self, name: Option<String>) -> Self {
		self.name = name;
		self
	}

	fn name(&self) -> &Option<String> {
		&self.name
	}
}

impl utility::BuildFromDevice for Builder {
	type Output = View;
	/// Creates a [`View`] object, thereby consuming the info.
	/// The created [`View`] will use the same format the [`Image`] uses,
	/// to garuntee fewer user-error bugs.
	fn build(self, device: &sync::Arc<logical::Device>) -> utility::Result<Self::Output> {
		use backend::version::DeviceV1_0;
		let image = self.image.upgrade().unwrap();
		let info = backend::vk::ImageViewCreateInfo::builder()
			.image(**image)
			.view_type(self.view_type)
			.format(image.format())
			.components(self.components)
			.subresource_range(self.subresource_range.into())
			.build();
		let vk = unsafe { device.create_image_view(&info, None) }?;
		let view = View::from(device.clone(), image, vk);
		self.set_object_name(device, &view);
		Ok(view)
	}
}
