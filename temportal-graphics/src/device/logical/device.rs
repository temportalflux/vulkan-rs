use crate::{
	backend, command,
	device::logical,
	image, instance,
	utility::{self, VulkanObject},
};
use std::rc::Rc;

/// A wrapper for a [`Vulkan LogicalDevice`](backend::Device),
/// which can send logical commands to the hardware.
pub struct Device {
	swapchain: backend::extensions::khr::Swapchain,
	internal: backend::Device,
}

impl Device {
	/// The internal constructor. Users should use [`Info.create_object`](struct.Info.html#method.create_object) to create a vulkan instance.
	pub fn from(instance: &instance::Instance, internal: backend::Device) -> Device {
		Device {
			swapchain: backend::extensions::khr::Swapchain::new(instance.unwrap(), &internal),
			internal,
		}
	}

	pub fn get_queue(device: &Rc<Self>, queue_family_index: usize) -> logical::Queue {
		use backend::version::DeviceV1_0;
		let vk = unsafe {
			device
				.unwrap()
				.get_device_queue(queue_family_index as u32, /*queue index*/ 0)
		};
		logical::Queue::from(device.clone(), vk, queue_family_index)
	}

	pub fn wait_for(
		&self,
		fence: &command::Fence,
		wait_for_all: bool,
		timeout: u64,
	) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		let fences = [*fence.unwrap()];
		utility::as_vulkan_error(unsafe {
			self.internal
				.wait_for_fences(&fences, wait_for_all, timeout)
		})
	}

	pub fn reset_fences(&self, fences: &[&command::Fence]) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		let fences = fences.iter().map(|f| *f.unwrap()).collect::<Vec<_>>();
		utility::as_vulkan_error(unsafe { self.internal.reset_fences(&fences[..]) })
	}

	pub fn wait_until_idle(&self) -> utility::Result<()> {
		use backend::version::DeviceV1_0;
		utility::as_vulkan_error(unsafe { self.internal.device_wait_idle() })
	}
}

/// A trait exposing the internal value for the wrapped [`backend::Device`].
/// Crates using `temportal_graphics` should NOT use this.
impl utility::VulkanObject<backend::Device> for Device {
	fn unwrap(&self) -> &backend::Device {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::Device {
		&mut self.internal
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
		unsafe { self.internal.destroy_image(*obj.unwrap(), None) };
		Ok(())
	}
}
