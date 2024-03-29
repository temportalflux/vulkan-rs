use crate::{
	backend,
	device::{
		logical,
		swapchain::{khr::Swapchain, Swapchain as SwapchainTrait, SwapchainBuilder},
	},
	flags::{
		format::Format, ColorSpace, CompositeAlpha, ImageUsageFlags, PresentMode, SharingMode,
		SurfaceTransform,
	},
	general::Surface,
	structs::Extent2D,
	utility,
};
use std::sync::{self, Arc};

/// Information used to construct a [`Swapchain`](crate::device::swapchain::khr::Swapchain).
#[derive(Clone)]
pub struct Builder {
	logical_device: Option<sync::Weak<logical::Device>>,
	surface: Option<sync::Weak<Surface>>,
	image_count: u32,
	pub(crate) image_format: Format,
	image_color_space: ColorSpace,
	pub(crate) image_extent: Extent2D,
	image_array_layer_count: u32,
	image_usage: ImageUsageFlags,
	sharing_mode: SharingMode,
	pre_transform: SurfaceTransform,
	composite_alpha: CompositeAlpha,
	present_mode: PresentMode,
	is_clipped: bool,
	name: String,
}

impl Default for Builder {
	fn default() -> Self {
		Self {
			logical_device: None,
			surface: None,
			image_count: 0,
			image_format: Format::UNDEFINED,
			image_color_space: ColorSpace::SRGB_NONLINEAR,
			image_extent: Extent2D::default(),
			image_array_layer_count: 0,
			image_usage: ImageUsageFlags::empty(),
			sharing_mode: SharingMode::EXCLUSIVE,
			pre_transform: SurfaceTransform::IDENTITY,
			composite_alpha: CompositeAlpha::OPAQUE,
			present_mode: PresentMode::MAILBOX,
			is_clipped: true,
			name: String::new(),
		}
	}
}

impl Builder {
	pub fn with_logical_device(mut self, device: &Arc<logical::Device>) -> Self {
		self.logical_device = Some(Arc::downgrade(&device));
		self
	}

	pub fn with_surface(mut self, surface: &Arc<Surface>) -> Self {
		self.surface = Some(Arc::downgrade(&surface));
		self
	}

	/// Mutates the builder to indicate the number of frame images to use.
	pub fn with_image_count(mut self, count: u32) -> Self {
		self.image_count = count;
		self
	}

	/// Mutates the builder to set the format of the frame images.
	pub fn with_image_format(mut self, format: Format) -> Self {
		self.image_format = format;
		self
	}

	/// Mutates the builder to set the color space of the frame images.
	pub fn with_image_color_space(mut self, color_space: ColorSpace) -> Self {
		self.image_color_space = color_space;
		self
	}

	/// Mutates the builder to set the extent/size/dimensions of the frame images.
	pub fn with_image_extent(mut self, extent: Extent2D) -> Self {
		self.image_extent = extent;
		self
	}

	/// Returns the extent/size/dimensions of the frame images.
	pub fn image_extent(&self) -> &Extent2D {
		&self.image_extent
	}

	/// Mutates the build to specify the number of array layers in each frame image.
	pub fn with_image_array_layer_count(mut self, layer_count: u32) -> Self {
		self.image_array_layer_count = layer_count;
		self
	}

	/// Mutates the build to specify the usage of the frame images.
	pub fn with_image_usage(mut self, usage: ImageUsageFlags) -> Self {
		self.image_usage = usage;
		self
	}

	/// Mutates the build to specify the sharing mode of the frame images.
	pub fn with_image_sharing_mode(mut self, mode: SharingMode) -> Self {
		self.sharing_mode = mode;
		self
	}

	pub fn with_pre_transform(mut self, transform: SurfaceTransform) -> Self {
		self.pre_transform = transform;
		self
	}

	pub fn with_composite_alpha(mut self, composite_alpha: CompositeAlpha) -> Self {
		self.composite_alpha = composite_alpha;
		self
	}

	pub fn with_is_clipped(mut self, is_clipped: bool) -> Self {
		self.is_clipped = is_clipped;
		self
	}

	pub fn with_present_mode(mut self, present_mode: PresentMode) -> Self {
		self.present_mode = present_mode;
		self
	}
}

impl SwapchainBuilder for Builder {
	fn image_count(&self) -> usize {
		self.image_count as usize
	}

	fn image_format(&self) -> Format {
		self.image_format
	}

	fn set_image_extent(&mut self, resolution: Extent2D) {
		self.image_extent = resolution;
	}

	fn image_extent(&self) -> &Extent2D {
		&self.image_extent
	}

	fn set_surface_transform(&mut self, transform: SurfaceTransform) {
		self.pre_transform = transform;
	}

	fn set_present_mode(&mut self, mode: PresentMode) {
		self.present_mode = mode;
	}

	fn build(
		&self,
		old: Option<Box<dyn SwapchainTrait + 'static + Send + Sync>>,
	) -> anyhow::Result<Box<dyn SwapchainTrait + 'static + Send + Sync>> {
		use utility::HandledObject;
		let device = self.logical_device.as_ref().unwrap().upgrade().unwrap();
		let surface = self.surface.as_ref().unwrap().upgrade().unwrap();
		let old_khr = old.as_ref().map(|chain| chain.as_khr()).flatten();
		let old = old_khr
			.map(|khr| **khr)
			.unwrap_or(backend::vk::SwapchainKHR::null());
		let info = backend::vk::SwapchainCreateInfoKHR::builder()
			.surface(**surface)
			.min_image_count(self.image_count)
			.image_format(self.image_format)
			.image_color_space(self.image_color_space)
			.image_extent(self.image_extent)
			.image_array_layers(self.image_array_layer_count)
			.image_usage(self.image_usage)
			.image_sharing_mode(self.sharing_mode)
			.pre_transform(self.pre_transform)
			.composite_alpha(self.composite_alpha)
			.present_mode(self.present_mode)
			.clipped(self.is_clipped)
			.old_swapchain(old)
			.build();
		let vk = unsafe { device.unwrap_swapchain().create_swapchain(&info, None) }?;
		let swapchain = Swapchain::from(device.clone(), vk, self.clone());
		device.set_object_name_logged(&swapchain.create_name(self.name.as_str()));
		Ok(Box::new(swapchain))
	}
}

impl utility::NameableBuilder for Builder {
	fn set_name(&mut self, name: impl Into<String>) {
		self.name = name.into();
	}

	fn name(&self) -> &String {
		&self.name
	}
}
