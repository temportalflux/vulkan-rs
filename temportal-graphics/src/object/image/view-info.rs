use crate::{
	device::logical,
	flags::{ComponentSwizzle, Format, ImageViewType},
	image,
	structs::{ComponentMapping, ImageSubresourceRange},
	utility::{self, VulkanInfo, VulkanObject},
};
use erupt;

pub struct ViewInfo {
	view_type: ImageViewType,
	format: Format,
	components: ComponentMapping,
	subresource_range: ImageSubresourceRange,
}

impl ViewInfo {
	pub fn new() -> ViewInfo {
		ViewInfo {
			view_type: ImageViewType::_2D,
			format: Format::UNDEFINED,
			components: ComponentMapping {
				r: ComponentSwizzle::R,
				g: ComponentSwizzle::G,
				b: ComponentSwizzle::B,
				a: ComponentSwizzle::A,
			},
			subresource_range: ImageSubresourceRange::default(),
		}
	}

	pub fn set_view_type(mut self, view_type: ImageViewType) -> Self {
		self.view_type = view_type;
		self
	}

	pub fn set_format(mut self, format: Format) -> Self {
		self.format = format;
		self
	}

	pub fn set_components(mut self, components: ComponentMapping) -> Self {
		self.components = components;
		self
	}

	pub fn set_subresource_range(mut self, subresource_range: ImageSubresourceRange) -> Self {
		self.subresource_range = subresource_range;
		self
	}
}

impl VulkanInfo<erupt::vk::ImageViewCreateInfo> for ViewInfo {
	/// Converts the [`ViewInfo`] into the [`erupt::vk::ImageViewCreateInfo`] struct
	/// used to create a [`image::View`].
	fn to_vk(&self) -> erupt::vk::ImageViewCreateInfo {
		erupt::vk::ImageViewCreateInfoBuilder::new()
			.view_type(self.view_type)
			.format(self.format)
			.components(self.components)
			.subresource_range(self.subresource_range)
			.build()
	}
}

impl ViewInfo {
	/// Creates an [`image::View`] object, thereby consuming the info.
	pub fn create_object(
		&mut self,
		device: &logical::Device,
		image: &image::Image,
	) -> Result<image::View, utility::Error> {
		let mut info = self.to_vk();
		info.image = *image.unwrap() as _;
		Ok(image::View::from(device.create_image_view(info)?))
	}
}
