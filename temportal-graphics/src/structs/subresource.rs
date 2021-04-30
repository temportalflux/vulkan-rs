pub use crate::{backend, flags::ImageAspect, utility};

pub struct Range {
	aspect: ImageAspect,
	mip_level_start: u32,
	mip_level_end_excl: u32,
	array_layer_start: u32,
	array_layer_end_excl: u32,
}

impl Default for Range {
	fn default() -> Range {
		Range {
			aspect: ImageAspect::empty(),
			mip_level_start: 0,
			mip_level_end_excl: 1,
			array_layer_start: 0,
			array_layer_end_excl: 1,
		}
	}
}

impl Range {
	pub fn with_aspect(mut self, aspect: ImageAspect) -> Self {
		self.aspect |= aspect;
		self
	}

	pub fn mips(mut self, range: impl std::ops::RangeBounds<u32>) -> Self {
		self.mip_level_start = match range.start_bound() {
			std::ops::Bound::Included(&i) => i,
			std::ops::Bound::Excluded(&i) => i + 1,
			std::ops::Bound::Unbounded => 0,
		};
		self.mip_level_end_excl = match range.end_bound() {
			std::ops::Bound::Included(&i) => i + 1,
			std::ops::Bound::Excluded(&i) => i,
			std::ops::Bound::Unbounded => 0,
		};
		self
	}

	pub fn layers(mut self, range: impl std::ops::RangeBounds<u32>) -> Self {
		self.array_layer_start = match range.start_bound() {
			std::ops::Bound::Included(&i) => i,
			std::ops::Bound::Excluded(&i) => i + 1,
			std::ops::Bound::Unbounded => 0,
		};
		self.array_layer_end_excl = match range.end_bound() {
			std::ops::Bound::Included(&i) => i + 1,
			std::ops::Bound::Excluded(&i) => i,
			std::ops::Bound::Unbounded => 0,
		};
		self
	}
}

impl utility::VulkanInfo<backend::vk::ImageSubresourceRange> for Range {
	fn to_vk(&self) -> backend::vk::ImageSubresourceRange {
		backend::vk::ImageSubresourceRange::builder()
			.aspect_mask(self.aspect)
			.base_mip_level(self.mip_level_start)
			.level_count(self.mip_level_end_excl - self.mip_level_start)
			.base_array_layer(self.array_layer_start)
			.layer_count(self.array_layer_end_excl - self.array_layer_start)
			.build()
	}
}

pub struct Layers {
	aspect: ImageAspect,
	mip_level: u32,
	array_layer_start: u32,
	array_layer_end_excl: u32,
}

impl Default for Layers {
	fn default() -> Layers {
		Layers {
			aspect: ImageAspect::empty(),
			mip_level: 0,
			array_layer_start: 0,
			array_layer_end_excl: 1,
		}
	}
}

impl Layers {
	pub fn with_aspect(mut self, aspect: ImageAspect) -> Self {
		self.aspect |= aspect;
		self
	}

	pub fn mip(mut self, level: u32) -> Self {
		self.mip_level = level;
		self
	}

	pub fn layers(mut self, range: impl std::ops::RangeBounds<u32>) -> Self {
		self.array_layer_start = match range.start_bound() {
			std::ops::Bound::Included(&i) => i,
			std::ops::Bound::Excluded(&i) => i + 1,
			std::ops::Bound::Unbounded => 0,
		};
		self.array_layer_end_excl = match range.end_bound() {
			std::ops::Bound::Included(&i) => i + 1,
			std::ops::Bound::Excluded(&i) => i,
			std::ops::Bound::Unbounded => 0,
		};
		self
	}
}

impl utility::VulkanInfo<backend::vk::ImageSubresourceLayers> for Layers {
	fn to_vk(&self) -> backend::vk::ImageSubresourceLayers {
		backend::vk::ImageSubresourceLayers::builder()
			.aspect_mask(self.aspect)
			.mip_level(self.mip_level)
			.base_array_layer(self.array_layer_start)
			.layer_count(self.array_layer_end_excl - self.array_layer_start)
			.build()
	}
}
