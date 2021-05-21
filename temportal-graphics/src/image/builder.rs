use crate::{
	alloc, backend,
	flags::{Format, ImageLayout, ImageTiling, ImageType, ImageUsage, SampleCount, SharingMode},
	image::Image,
	structs::Extent3D,
	utility,
};
use std::sync;
use temportal_math::Vector;

/// The builder for [`Image`] objects.
#[derive(Clone)]
pub struct Builder {
	/// The allocation information/builder for allocating the image.
	mem_info: alloc::Builder,
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
			mem_info: alloc::Builder::default(),
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
	/// Mutates the builder to include the memory allocation information.
	pub fn with_alloc(mut self, mem_info: alloc::Builder) -> Self {
		self.mem_info = mem_info;
		self
	}

	/// Mutates the builder to include an explicit dimensions/size of the image object.
	pub fn with_size(mut self, size: Vector<usize, 3>) -> Self {
		self.extent = Extent3D {
			width: size.x() as u32,
			height: size.y() as u32,
			depth: size.z() as u32,
		};
		self
	}

	/// Returns the size of the image to be allocated.
	pub(crate) fn size(&self) -> Vector<usize, 3> {
		Vector::new([
			self.extent.width as usize,
			self.extent.height as usize,
			self.extent.depth as usize,
		])
	}

	/// Mutates the builder to use a specific format when creating the image.
	pub fn with_format(mut self, format: Format) -> Self {
		self.format = format;
		self
	}

	/// Returns the format of the image to be allocated.
	pub(crate) fn format(&self) -> Format {
		self.format
	}

	/// Mutates the builder to use a specific tiling form.
	/// Uses [`optimal`](ImageTiling::OPTIMAL) tiling by default.
	pub fn with_tiling(mut self, tiling: ImageTiling) -> Self {
		self.tiling = tiling;
		self
	}

	/// Mutates the builder to include a flag indicating how the image will be used.
	/// Can be called multiple times with different flags to include each flag.
	pub fn with_usage(mut self, usage: ImageUsage) -> Self {
		self.usage |= usage;
		self
	}
	
	/// Creates an [`Image`] object, thereby consuming the info.
	pub fn build(self, allocator: &sync::Arc<alloc::Allocator>) -> utility::Result<Image> {
		let alloc_create_info = self.mem_info.clone().into();
		let (internal, alloc_handle, _alloc_info) = utility::as_alloc_error(
			allocator.create_image(&self.clone().into(), &alloc_create_info),
		)?;
		Ok(Image::new(
			allocator.clone(),
			internal,
			Some(alloc_handle),
			Some(self),
		))
	}
}

impl Into<backend::vk::ImageCreateInfo> for Builder {
	fn into(self) -> backend::vk::ImageCreateInfo {
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
