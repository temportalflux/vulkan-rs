use crate::{
	backend,
	device::logical,
	flags,
	utility::{self, VulkanObject},
};

use std::rc::Rc;

pub struct Semaphore {
	internal: backend::vk::Semaphore,
	device: Rc<logical::Device>,
}

impl Semaphore {
	pub fn new(device: &Rc<logical::Device>) -> utility::Result<Semaphore> {
		use backend::version::DeviceV1_0;
		let info = backend::vk::SemaphoreCreateInfo::builder().build();
		let vk =
			utility::as_vulkan_error(unsafe { device.unwrap().create_semaphore(&info, None) })?;
		Ok(Semaphore::from(device.clone(), vk))
	}

	pub fn from(device: Rc<logical::Device>, internal: backend::vk::Semaphore) -> Semaphore {
		Semaphore { device, internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Semaphore`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::Semaphore> for Semaphore {
	fn unwrap(&self) -> &backend::vk::Semaphore {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Semaphore {
		&mut self.internal
	}
}

impl Drop for Semaphore {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.unwrap().destroy_semaphore(self.internal, None) };
	}
}

pub struct Fence {
	internal: backend::vk::Fence,
	device: Rc<logical::Device>,
}

impl Fence {
	pub fn new(device: &Rc<logical::Device>, state: flags::FenceState) -> utility::Result<Fence> {
		use backend::version::DeviceV1_0;
		let info = backend::vk::FenceCreateInfo::builder().flags(state).build();
		let vk = utility::as_vulkan_error(unsafe { device.unwrap().create_fence(&info, None) })?;
		Ok(Fence::from(device.clone(), vk))
	}

	pub fn from(device: Rc<logical::Device>, internal: backend::vk::Fence) -> Fence {
		Fence { device, internal }
	}
}

/// A trait exposing the internal value for the wrapped [`backend::vk::Fence`].
/// Crates using `temportal_graphics` should NOT use this.
impl VulkanObject<backend::vk::Fence> for Fence {
	fn unwrap(&self) -> &backend::vk::Fence {
		&self.internal
	}
	fn unwrap_mut(&mut self) -> &mut backend::vk::Fence {
		&mut self.internal
	}
}

impl Drop for Fence {
	fn drop(&mut self) {
		use backend::version::DeviceV1_0;
		unsafe { self.device.unwrap().destroy_fence(self.internal, None) };
	}
}
