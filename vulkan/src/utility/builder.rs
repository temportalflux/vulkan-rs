use crate::{alloc, device::logical, utility};
use std::sync;

pub trait BuildFromAllocator {
	type Output;
	fn build(self, allocator: &sync::Arc<alloc::Allocator>) -> utility::Result<Self::Output>;
}

pub trait BuildFromDevice {
	type Output;
	fn build(&mut self, device: &sync::Arc<logical::Device>) -> utility::Result<Self::Output>;
}

pub trait NameableBuilder {
	fn with_name<T>(self, name: T) -> Self
	where
		T: Into<String>,
		Self: Sized,
	{
		self.with_optname(Some(name.into()))
	}

	fn with_optname(self, name: Option<String>) -> Self
	where
		Self: Sized;
	fn name(&self) -> &Option<String>;
}
