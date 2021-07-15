use crate::{
	backend,
	device::logical,
	flags::{BorderColor, CompareOp, Filter, SamplerAddressMode, SamplerMipmapMode},
	sampler, utility,
};
use std::sync;

pub struct Builder {
	magnification: Filter,
	minification: Filter,
	address_mode: [SamplerAddressMode; 3],
	border_color: BorderColor,
	mip_mode: SamplerMipmapMode,
	mip_lod_bias: f32,
	mip_lod_range: std::ops::Range<f32>,
	max_anisotropy: Option<f32>,
	compare_op: Option<CompareOp>,
	uses_unnormalized_coords: bool,
	name: Option<String>,
}

impl Default for Builder {
	fn default() -> Builder {
		Builder {
			magnification: Filter::LINEAR,
			minification: Filter::LINEAR,
			address_mode: [SamplerAddressMode::CLAMP_TO_BORDER; 3],
			border_color: BorderColor::INT_OPAQUE_BLACK,
			mip_mode: SamplerMipmapMode::LINEAR,
			mip_lod_bias: 0.0,
			mip_lod_range: 0.0..0.0,
			max_anisotropy: None,
			compare_op: None,
			uses_unnormalized_coords: false,
			name: None,
		}
	}
}

impl Builder {
	pub fn with_magnification(mut self, filter: Filter) -> Self {
		self.magnification = filter;
		self
	}

	pub fn with_minification(mut self, filter: Filter) -> Self {
		self.minification = filter;
		self
	}

	pub fn with_address_modes(mut self, modes: [SamplerAddressMode; 3]) -> Self {
		self.address_mode = modes;
		self
	}

	pub fn with_border_color(mut self, color: BorderColor) -> Self {
		self.border_color = color;
		self
	}

	pub fn with_mips(
		mut self,
		mode: SamplerMipmapMode,
		lod_bias: f32,
		lod_range: std::ops::Range<f32>,
	) -> Self {
		self.mip_mode = mode;
		self.mip_lod_bias = lod_bias;
		self.mip_lod_range = lod_range;
		self
	}

	pub fn with_max_anisotropy(mut self, max: Option<f32>) -> Self {
		self.max_anisotropy = max;
		self
	}

	pub fn with_compare_op(mut self, op: Option<CompareOp>) -> Self {
		self.compare_op = op;
		self
	}

	pub fn unnormalized(mut self) -> Self {
		self.uses_unnormalized_coords = true;
		self
	}
}

impl utility::NameableBuilder for Builder {
	fn with_optname(mut self, name: Option<String>) -> Self {
		self.name = name;
		self
	}

	fn name(&self) -> &Option<String> {
		&self.name
	}
}

impl Builder {
	fn as_vk(&self) -> backend::vk::SamplerCreateInfo {
		backend::vk::SamplerCreateInfo::builder()
			.mag_filter(self.magnification)
			.min_filter(self.minification)
			.address_mode_u(self.address_mode[0])
			.address_mode_v(self.address_mode[1])
			.address_mode_w(self.address_mode[2])
			.mipmap_mode(self.mip_mode)
			.mip_lod_bias(self.mip_lod_bias)
			.min_lod(self.mip_lod_range.start)
			.max_lod(self.mip_lod_range.end)
			.anisotropy_enable(self.max_anisotropy.is_some())
			.max_anisotropy(self.max_anisotropy.unwrap_or(0.0))
			.compare_enable(self.compare_op.is_some())
			.compare_op(self.compare_op.unwrap_or(CompareOp::ALWAYS))
			.border_color(self.border_color)
			.unnormalized_coordinates(self.uses_unnormalized_coords)
			.build()
	}
}

impl utility::BuildFromDevice for Builder {
	type Output = sampler::Sampler;
	fn build(self, device: &sync::Arc<logical::Device>) -> utility::Result<Self::Output> {
		use backend::version::DeviceV1_0;
		let vk = unsafe { device.create_sampler(&self.as_vk(), None) }?;
		let sampler = sampler::Sampler::from(device.clone(), vk);
		self.set_object_name(device, &sampler);
		Ok(sampler)
	}
}
