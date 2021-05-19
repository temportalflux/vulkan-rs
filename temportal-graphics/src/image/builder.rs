use crate::{
	alloc, backend,
	flags::{Format, ImageLayout, ImageTiling, ImageType, ImageUsage, SampleCount, SharingMode},
	image,
	structs::Extent3D,
	utility::{self, VulkanInfo},
};
use std::sync;
use temportal_math::Vector;

pub struct Builder {
	pub mem_info: alloc::Info,
	pub image_type: ImageType,
	pub format: Format,
	pub extent: Extent3D,
	pub mip_levels: u32,
	pub array_layers: u32,
	pub samples: SampleCount,
	pub tiling: ImageTiling,
	pub usage: ImageUsage,
	pub sharing_mode: SharingMode,
	pub initial_layout: ImageLayout,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			mem_info: alloc::Info::default(),
			image_type: ImageType::TYPE_2D,
			format: Format::UNDEFINED,
			extent: Extent3D::default(),
			mip_levels: 1,
			array_layers: 1,
			samples: SampleCount::TYPE_1,
			tiling: ImageTiling::OPTIMAL,
			usage: ImageUsage::default(),
			sharing_mode: SharingMode::EXCLUSIVE,
			initial_layout: ImageLayout::UNDEFINED,
		}
	}
}

impl Builder {
	pub fn size(&self) -> Vector<usize, 3> {
		Vector::new([
			self.extent.width as usize,
			self.extent.height as usize,
			self.extent.depth as usize,
		])
	}

	pub fn with_alloc(mut self, mem_info: alloc::Info) -> Self {
		self.mem_info = mem_info;
		self
	}

	pub fn with_size(mut self, size: Vector<usize, 3>) -> Self {
		self.extent = Extent3D {
			width: size.x() as u32,
			height: size.y() as u32,
			depth: size.z() as u32,
		};
		self
	}

	pub fn with_format(mut self, format: Format) -> Self {
		self.format = format;
		self
	}

	pub fn with_tiling(mut self, tiling: ImageTiling) -> Self {
		self.tiling = tiling;
		self
	}

	pub fn with_usage(mut self, usage: ImageUsage) -> Self {
		self.usage |= usage;
		self
	}
}

impl VulkanInfo<backend::vk::ImageCreateInfo> for Builder {
	/// Converts the [`Builder`] into the [`backend::vk::ImageCreateInfo`] struct
	/// used to create a [`image::Image`].
	fn to_vk(&self) -> backend::vk::ImageCreateInfo {
		backend::vk::ImageCreateInfo::builder()
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
	pub fn build(self, allocator: &sync::Arc<alloc::Allocator>) -> utility::Result<image::Image> {
		let image_info = self.to_vk();
		let alloc_create_info = self.mem_info.to_vk();
		let (internal, alloc_handle, alloc_info) =
			utility::as_alloc_error(allocator.create_image(&image_info, &alloc_create_info))?;
		Ok(image::Image::new(
			allocator.clone(),
			internal,
			Some(alloc_handle),
			Some(alloc_info),
			Some(self),
		))
	}
}
