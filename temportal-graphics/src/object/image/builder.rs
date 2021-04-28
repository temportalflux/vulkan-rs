use crate::{
	backend,
	flags::{Format, ImageLayout, ImageTiling, ImageType, ImageUsage, SampleCount, SharingMode},
	object::AllocationInfo,
	structs::Extent3D,
	utility::VulkanInfo,
};

pub struct Builder {
	mem_info: AllocationInfo,
	image_type: ImageType,
	format: Format,
	extent: Extent3D,
	mip_levels: u32,
	array_layers: u32,
	samples: SampleCount,
	tiling: ImageTiling,
	usage: ImageUsage,
	sharing_mode: SharingMode,
	initial_layout: ImageLayout,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			mem_info: AllocationInfo::default(),
			image_type: ImageType::_2D,
			format: Format::UNDEFINED,
			extent: Extent3D::default(),
			mip_levels: 1,
			array_layers: 1,
			samples: SampleCount::_1,
			tiling: ImageTiling::OPTIMAL,
			usage: ImageUsage::default(),
			sharing_mode: SharingMode::EXCLUSIVE,
			initial_layout: ImageLayout::UNDEFINED,
		}
	}
}

impl Builder {}

impl VulkanInfo<backend::vk::ImageCreateInfo> for Builder {
	/// Converts the [`Builder`] into the [`backend::vk::ImageCreateInfo`] struct
	/// used to create a [`image::Image`].
	fn to_vk(&self) -> backend::vk::ImageCreateInfo {
		backend::vk::ImageCreateInfoBuilder::new()
			.image_type(self.image_type)
			.format(self.format)
			.extent(self.extent)
			.mip_levels(self.mip_levels)
			.array_layers(self.array_layers)
			.samples(self.samples)
			.tiling(self.tiling)
			.usage(self.usage)
			.sharing_mode(self.sharing_mode)
			.initial_layout(self.initial_layout)
			.build()
	}
}

impl Builder {
	/// Creates an [`image::Image`] object, thereby consuming the info.
	pub fn create_object(self) -> Result<(), ()> {
		Ok(())
	}
}
