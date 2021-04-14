pub trait VulkanInfo<T> {
	fn to_vk(&mut self) -> T;
}
