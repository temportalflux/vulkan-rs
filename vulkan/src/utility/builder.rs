use crate::{alloc, device::logical, utility::HandledObject};
use std::sync;

pub trait BuildFromAllocator {
	type Output;

	fn build(self, allocator: &sync::Arc<alloc::Allocator>) -> anyhow::Result<Self::Output>;

	fn set_object_name(&self, allocator: &sync::Arc<alloc::Allocator>, handled: &Self::Output)
	where
		Self: NameableBuilder,
		Self::Output: HandledObject,
	{
		if let Some(device) = allocator.logical() {
			device.set_object_name_logged(&handled.create_name(self.name().as_str()));
		}
	}
}

pub trait BuildFromDevice {
	type Output;
	fn build(self, device: &sync::Arc<logical::Device>) -> anyhow::Result<Self::Output>;

	fn set_object_name(&self, device: &sync::Arc<logical::Device>, handled: &Self::Output)
	where
		Self: NameableBuilder,
		Self::Output: HandledObject,
	{
		device.set_object_name_logged(&handled.create_name(self.name().as_str()));
	}
}

pub trait NameableBuilder {
	fn with_name<T>(mut self, name: T) -> Self
	where
		T: Into<String>,
		Self: Sized,
	{
		self.set_name(name);
		self
	}

	fn set_name(&mut self, name: impl Into<String>)
	where
		Self: Sized;

	fn name(&self) -> &String;
}
