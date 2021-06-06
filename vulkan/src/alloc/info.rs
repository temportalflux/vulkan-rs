use crate::flags::{CreateAllocation, MemoryProperty, MemoryUsage};

/// Builder-pattern struct used to construct/allocate a [`graphics object`](crate::alloc::Object).
#[derive(Clone)]
pub struct Builder {
	mem_usage: MemoryUsage,
	alloc_flags: CreateAllocation,
	required_properties: MemoryProperty,
	preferred_properties: MemoryProperty,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			mem_usage: MemoryUsage::Unknown,
			alloc_flags: CreateAllocation::NONE,
			required_properties: MemoryProperty::empty(),
			preferred_properties: MemoryProperty::empty(),
		}
	}
}

impl Builder {
	/// Sets the intended usage of the memory (i.e. if the CPU and/or GPU can access the data).
	pub fn with_usage(mut self, usage: MemoryUsage) -> Self {
		self.mem_usage = usage;
		self
	}

	/// Inserts an allocation flag to mark the allocation for a specific creation type.
	/// Not often used, see [`CreateAllocation`] for more info on what flags can be used.
	pub fn with_alloc_flag(mut self, flag: CreateAllocation) -> Self {
		self.alloc_flags.insert(flag);
		self
	}

	/// Inserts a flag indicating that a given memory property needs to be supported for the allocation.
	/// This can be used to ensure that an object is local to the graphics device, or has i/o coherency for example.
	///
	/// See [`MemoryProperty`] for a full list of the valid property flags.
	pub fn requires(mut self, property: MemoryProperty) -> Self {
		self.required_properties |= property;
		self
	}

	/// Inserts a flag indicating that a given memory property should be supported if possible,
	/// but is not required in order for the allocation to be successful.
	/// This can be used to indicate to the allocator that it'd be nice if the allocation was
	/// [`Lazily Allocated`](MemoryProperty::LAZILY_ALLOCATED),
	/// for example, but need not be true to actually make the allocation.
	///
	/// See [`MemoryProperty`] for a full list of the valid property flags.
	pub fn prefers(mut self, property: MemoryProperty) -> Self {
		self.preferred_properties |= property;
		self
	}
}

impl Into<vk_mem::AllocationCreateInfo> for Builder {
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
