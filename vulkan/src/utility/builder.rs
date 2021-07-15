use crate::{
	alloc,
	device::logical,
	utility::{self, HandledObject},
};
use std::sync;

pub trait BuildFromAllocator {
	type Output;
	fn build(self, allocator: &sync::Arc<alloc::Allocator>) -> utility::Result<Self::Output>;

	fn set_object_name(&self, allocator: &sync::Arc<alloc::Allocator>, handled: &Self::Output)
	where
		Self: NameableBuilder,
		Self::Output: HandledObject,
	{
		if let Some(name) = self.name().as_ref() {
			if let Some(device) = allocator.logical() {
				device.set_object_name_logged(&handled.create_name(name.as_str()));
			}
		}
	}
}

pub trait BuildFromDevice {
	type Output;
	fn build(self, device: &sync::Arc<logical::Device>) -> utility::Result<Self::Output>;

	fn set_object_name(&self, device: &sync::Arc<logical::Device>, handled: &Self::Output)
	where
		Self: NameableBuilder,
		Self::Output: HandledObject,
	{
		if let Some(name) = self.name().as_ref() {
			device.set_object_name_logged(&handled.create_name(name.as_str()));
		}
	}
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
