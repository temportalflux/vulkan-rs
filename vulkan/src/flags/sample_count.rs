use crate::backend::vk::SampleCountFlags as VkEnum;
use enumset::EnumSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Serialize, Deserialize, enumset::EnumSetType)]
pub enum ImageSampleKind {
	Color,
	Depth,
	Stencil,
}

#[derive(Debug, Hash, PartialOrd, Ord, Serialize, Deserialize, enumset::EnumSetType)]
pub enum SampleCount {
	_1,
	_2,
	_4,
	_8,
	_16,
	_32,
	_64,
}

impl Default for SampleCount {
	fn default() -> Self {
		Self::_1
	}
}

impl Into<VkEnum> for SampleCount {
	fn into(self) -> VkEnum {
		match self {
			Self::_1 => VkEnum::TYPE_1,
			Self::_2 => VkEnum::TYPE_2,
			Self::_4 => VkEnum::TYPE_4,
			Self::_8 => VkEnum::TYPE_8,
			Self::_16 => VkEnum::TYPE_16,
			Self::_32 => VkEnum::TYPE_32,
			Self::_64 => VkEnum::TYPE_64,
		}
	}
}

impl SampleCount {
	pub fn as_set(flags: VkEnum) -> EnumSet<Self> {
		let mut set = EnumSet::empty();
		for sample in EnumSet::<Self>::all() {
			if flags.contains(sample.into()) {
				set.insert(sample);
			}
		}
		set
	}
}

#[cfg(test)]
mod sample_count {
	use super::*;

	#[test]
	fn empty() {
		assert_eq!(SampleCount::as_set(VkEnum::empty()), EnumSet::empty(),);
	}

	#[test]
	fn all() {
		assert_eq!(
			SampleCount::as_set(
				VkEnum::TYPE_1
					| VkEnum::TYPE_2 | VkEnum::TYPE_4
					| VkEnum::TYPE_8 | VkEnum::TYPE_16
					| VkEnum::TYPE_32 | VkEnum::TYPE_64
			),
			EnumSet::all(),
		);
	}
}
