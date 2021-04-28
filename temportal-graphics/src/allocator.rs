use crate::{
	device::{logical, physical},
	instance,
	utility::VulkanObject,
};
use std::rc::Rc;

pub struct Allocator {}

impl Allocator {
	pub fn create(
		instance: &Rc<instance::Instance>,
		physical: &Rc<physical::Device>,
		logical: &Rc<logical::Device>,
	) {
		//let info = vk_mem::AllocatorCreateInfo {
		//	instance: unsafe { ash::Instance::load(_, instance.unwrap().handle) },
		//	physical_device: physical.unwrap(),
		//	device: logical.unwrap(),
		//	.. Default::default()
		//};
	}
}
