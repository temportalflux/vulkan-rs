pub trait VulkanInfo<T> {
	fn to_vk(&self) -> T;
}
pub trait VulkanInfoMut<T> {
	fn to_vk(&mut self) -> T;
}
