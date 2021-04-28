use crate::{
	flags::{CreateAllocation, MemoryProperty, MemoryUsage},
	utility::VulkanInfo,
};

pub struct AllocationInfo {
	mem_usage: MemoryUsage,
	alloc_flags: CreateAllocation,
	required_properties: MemoryProperty,
	preferred_properties: MemoryProperty,
}

impl Default for AllocationInfo {
	fn default() -> AllocationInfo {
		AllocationInfo {
			mem_usage: MemoryUsage::Unknown,
			alloc_flags: CreateAllocation::NONE,
			required_properties: MemoryProperty::empty(),
			preferred_properties: MemoryProperty::empty(),
		}
	}
}

impl AllocationInfo {
	pub fn with_usage(mut self, usage: MemoryUsage) -> Self {
		self.mem_usage = usage;
		self
	}

	pub fn with_alloc_flag(mut self, flag: CreateAllocation) -> Self {
		self.alloc_flags.insert(flag);
		self
	}

	pub fn requires(mut self, property: MemoryProperty) -> Self {
		self.required_properties &= property;
		self
	}

	pub fn prefers(mut self, property: MemoryProperty) -> Self {
		self.preferred_properties &= property;
		self
	}
}

impl VulkanInfo<vk_mem::AllocationCreateInfo> for AllocationInfo {
	/// Converts the [`AllocationInfo`] into the [`vk_mem::AllocationCreateInfo`] struct.
	fn to_vk(&self) -> vk_mem::AllocationCreateInfo {
		vk_mem::AllocationCreateInfo {
			//usage: self.mem_usage,
			//flags: self.alloc_flags,
			//required_flags: ash::vk::MemoryPropertyFlags::from_raw(self.required_properties.bits()),
			//preferred_flags: ash::vk::MemoryPropertyFlags::from_raw(self.preferred_properties.bits()),
			..Default::default()
		}
	}
}
