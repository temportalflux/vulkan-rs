use crate::backend::vk::SampleCountFlags as VkEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
