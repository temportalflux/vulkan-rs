use crate::{
	backend, command,
	device::logical,
	image,
	instance::Instance,
	utility::{self},
};
use std::sync;

/// A wrapper for a [`Vulkan LogicalDevice`](backend::Device),
/// which can send logical commands to the hardware.
pub struct Device {
	swapchain: backend::extensions::khr::Swapchain,
	internal: backend::Device,
	instance: sync::Weak<Instance>,
}

impl Device {
	/// The internal constructor. Users should use [`Info.create_object`](struct.Info.html#method.create_object) to create a vulkan instance.
	pub(crate) fn from(instance: &sync::Arc<Instance>, internal: backend::Device) -> Device {
		Device {
			instance: sync::Arc::downgrade(&instance),
			swapchain: backend::extensions::khr::Swapchain::new(&***instance, &internal),
			internal,
		}
	}

	pub fn get_queue(device: &sync::Arc<Self>, queue_family_index: usize) -> logical::Queue {
		use backend::version::DeviceV1_0;
		let vk = unsafe {
			device.get_device_queue(queue_family_index as u32, /*queue index*/ 0)
		};
		logical::Queue::from(device.clone(), vk, queue_family_index)
	}

	pub fn wait_for(&self, fence: &command::Fence, timeout: u64) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		let fences = [**fence];
		Ok(unsafe { self.internal.wait_for_fences(&fences, true, timeout) }?)
	}

	pub fn reset_fences(&self, fences: &[&command::Fence]) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		let fences = fences.iter().map(|f| ***f).collect::<Vec<_>>();
		Ok(unsafe { self.internal.reset_fences(&fences[..]) }?)
	}

	pub fn wait_until_idle(&self) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		Ok(unsafe { self.internal.device_wait_idle() }?)
	}

	pub fn set_object_name(&self, name: &utility::ObjectName) -> utility::Result<()> {
		if let Some(instance) = self.instance.upgrade() {
			return Ok(unsafe {
				instance
					.debug_utils()
					.debug_utils_set_object_name(self.internal.handle(), &name.as_vk())
			}?);
		}
		Ok(())
	}

	pub fn set_object_name_logged(&self, name: &utility::ObjectName) {
		if let Err(err) = self.set_object_name(name) {
			log::error!(
				target: crate::LOG,
				"Failed to apply debug-utils object_name \"{}\"; {}",
				name,
				err
			);
		}
	}
}

impl std::ops::Deref for Device {
	type Target = backend::Device;
	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl Device {
	pub fn unwrap_swapchain(&self) -> &backend::extensions::khr::Swapchain {
		&self.swapchain
	}
}

impl Drop for Device {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe {
			self.internal.destroy_device(None);
		}
	}
}

#[doc(hidden)]
impl image::Owner for Device {
	fn destroy(&self, obj: &image::Image, _: Option<&vk_mem::Allocation>) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		unsafe { self.internal.destroy_image(**obj, None) };
		Ok(())
	}
}

impl utility::NamableObject for Device {
	fn kind(&self) -> backend::vk::ObjectType {
		<backend::vk::Device as backend::vk::Handle>::TYPE
	}

	fn handle(&self) -> u64 {
		use backend::vk::Handle;
		self.internal.handle().as_raw()
	}
}
