use crate::{
	device::{logical, swapchain::*},
	general::Surface,
	utility::VulkanObject,
	ColorSpace, CompositeAlpha, Extent2D, Format, ImageUsageFlags, PresentMode, SharingMode,
	SurfaceTransform,
};
use erupt;
use temportal_math::Vector;

pub struct Info {
	image_count: u32,
	image_format: Format,
	image_color_space: ColorSpace,
	image_extent: Extent2D,
	image_array_layer_count: u32,
	image_usage: ImageUsageFlags,
	sharing_mode: SharingMode,
	pre_transform: SurfaceTransform,
	composite_alpha: CompositeAlpha,
	present_mode: PresentMode,
	is_clipped: bool,
}

impl Info {
	pub fn new() -> Info {
		Info {
			image_count: 0,
			image_format: Format::UNDEFINED,
			image_color_space: ColorSpace::SRGB_NONLINEAR_KHR,
			image_extent: Extent2D::default(),
			image_array_layer_count: 0,
			image_usage: ImageUsageFlags::empty(),
			sharing_mode: SharingMode::EXCLUSIVE,
			pre_transform: SurfaceTransform::IDENTITY_KHR,
			composite_alpha: CompositeAlpha::OPAQUE_KHR,
			present_mode: PresentMode::MAILBOX_KHR,
			is_clipped: true,
		}
	}

	pub fn set_image_count(mut self, count: u32) -> Self {
		self.image_count = count;
		self
	}

	pub fn set_image_format(mut self, format: Format) -> Self {
		self.image_format = format;
		self
	}

	pub fn set_image_color_space(mut self, color_space: ColorSpace) -> Self {
		self.image_color_space = color_space;
		self
	}

	pub fn set_image_extent(mut self, extent: Extent2D) -> Self {
		self.image_extent = extent;
		self
	}

	pub fn set_image_extent_vec(self, extent: Vector<u32, 2>) -> Self {
		self.set_image_extent(Extent2D {
			width: extent.x(),
			height: extent.y(),
		})
	}

	pub fn set_image_array_layer_count(mut self, layer_count: u32) -> Self {
		self.image_array_layer_count = layer_count;
		self
	}

	pub fn set_image_usage(mut self, usage: ImageUsageFlags) -> Self {
		self.image_usage = usage;
		self
	}

	pub fn set_image_sharing_mode(mut self, mode: SharingMode) -> Self {
		self.sharing_mode = mode;
		self
	}

	pub fn set_pre_transform(mut self, transform: SurfaceTransform) -> Self {
		self.pre_transform = transform;
		self
	}

	pub fn set_composite_alpha(mut self, composite_alpha: CompositeAlpha) -> Self {
		self.composite_alpha = composite_alpha;
		self
	}

	pub fn set_is_clipped(mut self, is_clipped: bool) -> Self {
		self.is_clipped = is_clipped;
		self
	}

	pub fn set_present_mode(mut self, present_mode: PresentMode) -> Self {
		self.present_mode = present_mode;
		self
	}

	pub fn create_object(&mut self, device: &logical::Device, surface: &Surface) -> Swapchain {
		Swapchain::new(
			device.create_swapchain(
				erupt::vk::SwapchainCreateInfoKHRBuilder::new()
					.surface(*surface.unwrap())
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
					.old_swapchain(erupt::vk::SwapchainKHR::null())
					.build(),
			),
		)
	}
}