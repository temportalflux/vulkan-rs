use crate::{
	backend,
	device::{logical, physical, swapchain::*},
	flags::{
		ColorSpace, CompositeAlpha, Format, ImageUsageFlags, PresentMode, SharingMode,
		SurfaceTransform,
	},
	general::Surface,
	structs::Extent2D,
	utility::{self, VulkanObject},
};

use std::rc::Rc;
use temportal_math::Vector;

/// Information used to construct a [`Swapchain`](crate::device::swapchain::Swapchain).
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

impl Default for Info {
	fn default() -> Info {
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
}

impl Info {
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

	pub fn image_extent(&self) -> &Extent2D {
		&self.image_extent
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

	pub fn fill_from_physical(&mut self, physical: &physical::Device) {
		let surface_support = physical.query_surface_support();
		self.image_extent = surface_support.image_extent();
		self.pre_transform = surface_support.current_transform();
		self.present_mode = physical.selected_present_mode;
	}

	/// Creates the [`Swapchain`](crate::device::swapchain::Swapchain) object.
	pub fn create_object(
		&self,
		device: &Rc<logical::Device>,
		surface: &Surface,
		old: Option<&Swapchain>,
	) -> Result<Swapchain, utility::Error> {
		let vk = device.create_swapchain(
			backend::vk::SwapchainCreateInfoKHRBuilder::new()
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
				.old_swapchain(
					old.map_or(backend::vk::SwapchainKHR::null(), |chain| *chain.unwrap()),
				)
				.build(),
		)?;
		Ok(Swapchain::from(device.clone(), vk))
	}
}
