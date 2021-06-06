use crate::backend::vk::ColorComponentFlags as VkEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, enumset::EnumSetType)]
pub enum ColorComponent {
	R,
	G,
	B,
	A,
}

impl Into<VkEnum> for ColorComponent {
	fn into(self) -> VkEnum {
		match self {
			Self::R => VkEnum::R,
			Self::G => VkEnum::G,
			Self::B => VkEnum::B,
			Self::A => VkEnum::A,
		}
	}
}

impl ColorComponent {
	pub fn vecset(vec: &Vec<Self>) -> enumset::EnumSet<Self> {
		vec.iter()
			.fold(enumset::EnumSet::empty(), |mut set, value| {
				set.insert(*value);
				set
			})
	}
	pub fn fold(set: &enumset::EnumSet<Self>) -> VkEnum {
		set.iter()
			.fold(VkEnum::empty(), |vk, value| vk | value.into())
	}
}
