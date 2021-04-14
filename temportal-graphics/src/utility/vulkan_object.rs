pub trait VulkanObject<T> {
	fn unwrap(&self) -> &T;
	fn unwrap_mut(&mut self) -> &mut T;
}
