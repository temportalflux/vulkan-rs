use crate::flags::{CreateAllocation, MemoryProperty, MemoryUsage};

#[derive(Clone)]
pub struct Info {
	mem_usage: MemoryUsage,
	alloc_flags: CreateAllocation,
	required_properties: MemoryProperty,
	preferred_properties: MemoryProperty,
}

impl Default for Info {
	fn default() -> Info {
		Info {
			mem_usage: MemoryUsage::Unknown,
			alloc_flags: CreateAllocation::NONE,
			required_properties: MemoryProperty::empty(),
			preferred_properties: MemoryProperty::empty(),
		}
	}
}

impl Info {
	pub fn with_usage(mut self, usage: MemoryUsage) -> Self {
		self.mem_usage = usage;
		self
	}

	pub fn with_alloc_flag(mut self, flag: CreateAllocation) -> Self {
		self.alloc_flags.insert(flag);
		self
	}

	pub fn requires(mut self, property: MemoryProperty) -> Self {
		self.required_properties |= property;
		self
	}

	pub fn prefers(mut self, property: MemoryProperty) -> Self {
		self.preferred_properties |= property;
		self
	}
}

impl Into<vk_mem::AllocationCreateInfo> for Info {
	fn into(self) -> vk_mem::AllocationCreateInfo {
		vk_mem::AllocationCreateInfo {
			usage: self.mem_usage,
			flags: self.alloc_flags,
			required_flags: self.required_properties,
			preferred_flags: self.preferred_properties,
			memory_type_bits: 0,
			pool: None,
			user_data: None,
		}
	}
}
