use crate::{
	backend,
	device::logical,
	flags::{ComponentSwizzle, Format, ImageViewType},
	image, image_view,
	structs::{subresource, ComponentMapping},
	utility::{self, VulkanInfo, VulkanObject},
};

use std::rc::{Rc, Weak};

pub struct Builder {
	image: Weak<image::Image>,
	view_type: ImageViewType,
	format: Format,
	components: ComponentMapping,
	subresource_range: subresource::Range,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			image: Weak::new(),
			view_type: ImageViewType::TYPE_2D,
			format: Format::UNDEFINED,
			components: ComponentMapping {
				r: ComponentSwizzle::R,
				g: ComponentSwizzle::G,
				b: ComponentSwizzle::B,
				a: ComponentSwizzle::A,
			},
			subresource_range: subresource::Range::default(),
		}
	}
}

impl Builder {
	pub fn for_image(mut self, image: &Rc<image::Image>) -> Self {
		self.image = Rc::downgrade(image);
		self
	}

	pub fn with_view_type(mut self, view_type: ImageViewType) -> Self {
		self.view_type = view_type;
		self
	}

	pub fn with_format(mut self, format: Format) -> Self {
		self.format = format;
		self
	}

	pub fn with_components(mut self, components: ComponentMapping) -> Self {
		self.components = components;
		self
	}

	pub fn with_range(mut self, subresource_range: subresource::Range) -> Self {
		self.subresource_range = subresource_range;
		self
	}
}

impl VulkanInfo<backend::vk::ImageViewCreateInfo> for Builder {
	/// Converts the [`ViewInfo`] into the [`backend::vk::ImageViewCreateInfo`] struct
	/// used to create a [`image::View`].
	fn to_vk(&self) -> backend::vk::ImageViewCreateInfo {
		let image_rc = self.image.upgrade().unwrap();
		backend::vk::ImageViewCreateInfo::builder()
			.image(*image_rc.unwrap())
			.view_type(self.view_type)
			.format(self.format)
			.components(self.components)
			.subresource_range(self.subresource_range.to_vk())
			.build()
	}
}

impl Builder {
	/// Creates an [`image::View`] object, thereby consuming the info.
	pub fn build(&mut self, device: &Rc<logical::Device>) -> utility::Result<image_view::View> {
		use backend::version::DeviceV1_0;
		let info = self.to_vk();
		let vk =
			utility::as_vulkan_error(unsafe { device.unwrap().create_image_view(&info, None) })?;
		Ok(image_view::View::from(
			device.clone(),
			self.image.upgrade().unwrap(),
			vk,
		))
	}
}
