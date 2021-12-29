pub use crate::backend::vk::Format;
use crate::flags::ColorComponent;
use serde::{Deserialize, Serialize};

pub mod prelude {
	pub use super::Bits::*;
	pub use super::DataType::*;
	pub use crate::flags::ColorComponent::*;
}
use prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
	UnsignedNorm,
	SignedNorm,
	UnsignedScaled,
	SignedScaled,
	UnsignedInt,
	SignedInt,
	SRGB,
	SFloat,
	Block,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bits {
	Bit8,
	Bit16,
	Bit32,
	Bit64,
}

pub fn default() -> Format {
	Format::UNDEFINED
}

pub static SRGB_8BIT: Format = format(&[R, G, B, A], Bit8, SRGB);
pub static SRGB_8BIT_R: Format = format(&[R], Bit8, SRGB);

pub static VEC2: Format = format(&[R, G], Bit32, SFloat);
pub static VEC3: Format = format(&[R, G, B], Bit32, SFloat);
pub static VEC4: Format = format(&[R, G, B, A], Bit32, SFloat);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Components {
	color: Vec<ColorComponent>,
	bits: Bits,
	data: DataType,
}

impl Components {
	pub fn as_format(&self) -> Format {
		format(&self.color, self.bits, self.data)
	}
}

pub const fn format(comps: &[ColorComponent], bits: Bits, size: DataType) -> Format {
	match (comps, bits, size) {
		//R4G4_UNORM_PACK8
		//R4G4B4A4_UNORM_PACK16
		//B4G4R4A4_UNORM_PACK16
		//R5G6B5_UNORM_PACK16
		//B5G6R5_UNORM_PACK16
		//R5G5B5A1_UNORM_PACK16
		//B5G5R5A1_UNORM_PACK16
		//A1R5G5B5_UNORM_PACK16

		// 8-bit: Red
		([R], Bit8, UnsignedNorm) => Format::R8_UNORM,
		([R], Bit8, SignedNorm) => Format::R8_SNORM,
		([R], Bit8, UnsignedScaled) => Format::R8_USCALED,
		([R], Bit8, SignedScaled) => Format::R8_SSCALED,
		([R], Bit8, UnsignedInt) => Format::R8_UINT,
		([R], Bit8, SignedInt) => Format::R8_SINT,
		([R], Bit8, SRGB) => Format::R8_SRGB,

		// 8-bit: Red Green
		([R, G], Bit8, UnsignedNorm) => Format::R8G8_UNORM,
		([R, G], Bit8, SignedNorm) => Format::R8G8_SNORM,
		([R, G], Bit8, UnsignedScaled) => Format::R8G8_USCALED,
		([R, G], Bit8, SignedScaled) => Format::R8G8_SSCALED,
		([R, G], Bit8, UnsignedInt) => Format::R8G8_UINT,
		([R, G], Bit8, SignedInt) => Format::R8G8_SINT,
		([R, G], Bit8, SRGB) => Format::R8G8_SRGB,

		// 8-bit: Red Green Blue
		([R, G, B], Bit8, UnsignedNorm) => Format::R8G8B8_UNORM,
		([R, G, B], Bit8, SignedNorm) => Format::R8G8B8_SNORM,
		([R, G, B], Bit8, UnsignedScaled) => Format::R8G8B8_USCALED,
		([R, G, B], Bit8, SignedScaled) => Format::R8G8B8_SSCALED,
		([R, G, B], Bit8, UnsignedInt) => Format::R8G8B8_UINT,
		([R, G, B], Bit8, SignedInt) => Format::R8G8B8_SINT,
		([R, G, B], Bit8, SRGB) => Format::R8G8B8_SRGB,

		// 8-bit: Blue Green Red
		([B, G, R], Bit8, UnsignedNorm) => Format::B8G8R8_UNORM,
		([B, G, R], Bit8, SignedNorm) => Format::B8G8R8_SNORM,
		([B, G, R], Bit8, UnsignedScaled) => Format::B8G8R8_USCALED,
		([B, G, R], Bit8, SignedScaled) => Format::B8G8R8_SSCALED,
		([B, G, R], Bit8, UnsignedInt) => Format::B8G8R8_UINT,
		([B, G, R], Bit8, SignedInt) => Format::B8G8R8_SINT,
		([B, G, R], Bit8, SRGB) => Format::B8G8R8_SRGB,

		// 8-bit: Red Green Blue Alpha
		([R, G, B, A], Bit8, UnsignedNorm) => Format::R8G8B8A8_UNORM,
		([R, G, B, A], Bit8, SignedNorm) => Format::R8G8B8A8_SNORM,
		([R, G, B, A], Bit8, UnsignedScaled) => Format::R8G8B8A8_USCALED,
		([R, G, B, A], Bit8, SignedScaled) => Format::R8G8B8A8_SSCALED,
		([R, G, B, A], Bit8, UnsignedInt) => Format::R8G8B8A8_UINT,
		([R, G, B, A], Bit8, SignedInt) => Format::R8G8B8A8_SINT,
		([R, G, B, A], Bit8, SRGB) => Format::R8G8B8A8_SRGB,

		// 8-bit: Blue Green Red Alpha
		([B, G, R, A], Bit8, UnsignedNorm) => Format::B8G8R8A8_UNORM,
		([B, G, R, A], Bit8, SignedNorm) => Format::B8G8R8A8_SNORM,
		([B, G, R, A], Bit8, UnsignedScaled) => Format::B8G8R8A8_USCALED,
		([B, G, R, A], Bit8, SignedScaled) => Format::B8G8R8A8_SSCALED,
		([B, G, R, A], Bit8, UnsignedInt) => Format::B8G8R8A8_UINT,
		([B, G, R, A], Bit8, SignedInt) => Format::B8G8R8A8_SINT,
		([B, G, R, A], Bit8, SRGB) => Format::B8G8R8A8_SRGB,

		//A8B8G8R8_UNORM_PACK32,
		//A8B8G8R8_SNORM_PACK32,
		//A8B8G8R8_USCALED_PACK32,
		//A8B8G8R8_SSCALED_PACK32,
		//A8B8G8R8_UINT_PACK32,
		//A8B8G8R8_SINT_PACK32,
		//A8B8G8R8_SRGB_PACK32,
		//A2R10G10B10_UNORM_PACK32,
		//A2R10G10B10_SNORM_PACK32,
		//A2R10G10B10_USCALED_PACK32,
		//A2R10G10B10_SSCALED_PACK32,
		//A2R10G10B10_UINT_PACK32,
		//A2R10G10B10_SINT_PACK32,
		//A2B10G10R10_UNORM_PACK32,
		//A2B10G10R10_SNORM_PACK32,
		//A2B10G10R10_USCALED_PACK32,
		//A2B10G10R10_SSCALED_PACK32,
		//A2B10G10R10_UINT_PACK32,
		//A2B10G10R10_SINT_PACK32,

		// 16-bit: Red
		([R], Bit16, UnsignedNorm) => Format::R16_UNORM,
		([R], Bit16, SignedNorm) => Format::R16_SNORM,
		([R], Bit16, UnsignedScaled) => Format::R16_USCALED,
		([R], Bit16, SignedScaled) => Format::R16_SSCALED,
		([R], Bit16, UnsignedInt) => Format::R16_UINT,
		([R], Bit16, SignedInt) => Format::R16_SINT,
		([R], Bit16, SFloat) => Format::R16_SFLOAT,

		// 16-bit: Red Green
		([R, G], Bit16, UnsignedNorm) => Format::R16G16_UNORM,
		([R, G], Bit16, SignedNorm) => Format::R16G16_SNORM,
		([R, G], Bit16, UnsignedScaled) => Format::R16G16_USCALED,
		([R, G], Bit16, SignedScaled) => Format::R16G16_SSCALED,
		([R, G], Bit16, UnsignedInt) => Format::R16G16_UINT,
		([R, G], Bit16, SignedInt) => Format::R16G16_SINT,
		([R, G], Bit16, SFloat) => Format::R16G16_SFLOAT,

		// 16-bit: Red Green Blue
		([R, G, B], Bit16, UnsignedNorm) => Format::R16G16B16_UNORM,
		([R, G, B], Bit16, SignedNorm) => Format::R16G16B16_SNORM,
		([R, G, B], Bit16, UnsignedScaled) => Format::R16G16B16_USCALED,
		([R, G, B], Bit16, SignedScaled) => Format::R16G16B16_SSCALED,
		([R, G, B], Bit16, UnsignedInt) => Format::R16G16B16_UINT,
		([R, G, B], Bit16, SignedInt) => Format::R16G16B16_SINT,
		([R, G, B], Bit16, SFloat) => Format::R16G16B16_SFLOAT,

		// 16-bit: Red Green Blue Alpha
		([R, G, B, A], Bit16, UnsignedNorm) => Format::R16G16B16A16_UNORM,
		([R, G, B, A], Bit16, SignedNorm) => Format::R16G16B16A16_SNORM,
		([R, G, B, A], Bit16, UnsignedScaled) => Format::R16G16B16A16_USCALED,
		([R, G, B, A], Bit16, SignedScaled) => Format::R16G16B16A16_SSCALED,
		([R, G, B, A], Bit16, UnsignedInt) => Format::R16G16B16A16_UINT,
		([R, G, B, A], Bit16, SignedInt) => Format::R16G16B16A16_SINT,
		([R, G, B, A], Bit16, SFloat) => Format::R16G16B16A16_SFLOAT,

		// 32-bit: Red
		([R], Bit32, UnsignedInt) => Format::R32_UINT,
		([R], Bit32, SignedInt) => Format::R32_SINT,
		([R], Bit32, SFloat) => Format::R32_SFLOAT,

		// 32-bit: Red Green
		([R, G], Bit32, UnsignedInt) => Format::R32G32_UINT,
		([R, G], Bit32, SignedInt) => Format::R32G32_SINT,
		([R, G], Bit32, SFloat) => Format::R32G32_SFLOAT,

		// 32-bit: Red Green Blue
		([R, G, B], Bit32, UnsignedInt) => Format::R32G32B32_UINT,
		([R, G, B], Bit32, SignedInt) => Format::R32G32B32_SINT,
		([R, G, B], Bit32, SFloat) => Format::R32G32B32_SFLOAT,

		// 32-bit: Red Green Blue Alpha
		([R, G, B, A], Bit32, UnsignedInt) => Format::R32G32B32A32_UINT,
		([R, G, B, A], Bit32, SignedInt) => Format::R32G32B32A32_SINT,
		([R, G, B, A], Bit32, SFloat) => Format::R32G32B32A32_SFLOAT,

		// 64-bit: Red
		([R], Bit64, UnsignedInt) => Format::R64_UINT,
		([R], Bit64, SignedInt) => Format::R64_SINT,
		([R], Bit64, SFloat) => Format::R64_SFLOAT,

		// 64-bit: Red Green
		([R, G], Bit64, UnsignedInt) => Format::R64G64_UINT,
		([R, G], Bit64, SignedInt) => Format::R64G64_SINT,
		([R, G], Bit64, SFloat) => Format::R64G64_SFLOAT,

		// 64-bit: Red Green Blue
		([R, G, B], Bit64, UnsignedInt) => Format::R64G64B64_UINT,
		([R, G, B], Bit64, SignedInt) => Format::R64G64B64_SINT,
		([R, G, B], Bit64, SFloat) => Format::R64G64B64_SFLOAT,

		// 64-bit: Red Green Blue Alpha
		([R, G, B, A], Bit64, UnsignedInt) => Format::R64G64B64A64_UINT,
		([R, G, B, A], Bit64, SignedInt) => Format::R64G64B64A64_SINT,
		([R, G, B, A], Bit64, SFloat) => Format::R64G64B64A64_SFLOAT,

		//B10G11R11_UFLOAT_PACK32,
		//E5B9G9R9_UFLOAT_PACK32,
		//D16_UNORM,
		//X8_D24_UNORM_PACK32,
		//D32_SFLOAT,
		//S8_UINT,
		//D16_UNORM_S8_UINT,
		//D24_UNORM_S8_UINT,
		//D32_SFLOAT_S8_UINT,
		//BC1_RGB_UNORM_BLOCK,
		//BC1_RGB_SRGB_BLOCK,
		//BC1_RGBA_UNORM_BLOCK,
		//BC1_RGBA_SRGB_BLOCK,
		//BC2_UNORM_BLOCK,
		//BC2_SRGB_BLOCK,
		//BC3_UNORM_BLOCK,
		//BC3_SRGB_BLOCK,
		//BC4_UNORM_BLOCK,
		//BC4_SNORM_BLOCK,
		//BC5_UNORM_BLOCK,
		//BC5_SNORM_BLOCK,
		//BC6H_UFLOAT_BLOCK,
		//BC6H_SFLOAT_BLOCK,
		//BC7_UNORM_BLOCK,
		//BC7_SRGB_BLOCK,
		//ETC2_R8G8B8_UNORM_BLOCK,
		//ETC2_R8G8B8_SRGB_BLOCK,
		//ETC2_R8G8B8A1_UNORM_BLOCK,
		//ETC2_R8G8B8A1_SRGB_BLOCK,
		//ETC2_R8G8B8A8_UNORM_BLOCK,
		//ETC2_R8G8B8A8_SRGB_BLOCK,
		//EAC_R11_UNORM_BLOCK,
		//EAC_R11_SNORM_BLOCK,
		//EAC_R11G11_UNORM_BLOCK,
		//EAC_R11G11_SNORM_BLOCK,
		//ASTC_4X4_UNORM_BLOCK,
		//ASTC_4X4_SRGB_BLOCK,
		//ASTC_5X4_UNORM_BLOCK,
		//ASTC_5X4_SRGB_BLOCK,
		//ASTC_5X5_UNORM_BLOCK,
		//ASTC_5X5_SRGB_BLOCK,
		//ASTC_6X5_UNORM_BLOCK,
		//ASTC_6X5_SRGB_BLOCK,
		//ASTC_6X6_UNORM_BLOCK,
		//ASTC_6X6_SRGB_BLOCK,
		//ASTC_8X5_UNORM_BLOCK,
		//ASTC_8X5_SRGB_BLOCK,
		//ASTC_8X6_UNORM_BLOCK,
		//ASTC_8X6_SRGB_BLOCK,
		//ASTC_8X8_UNORM_BLOCK,
		//ASTC_8X8_SRGB_BLOCK,
		//ASTC_10X5_UNORM_BLOCK,
		//ASTC_10X5_SRGB_BLOCK,
		//ASTC_10X6_UNORM_BLOCK,
		//ASTC_10X6_SRGB_BLOCK,
		//ASTC_10X8_UNORM_BLOCK,
		//ASTC_10X8_SRGB_BLOCK,
		//ASTC_10X10_UNORM_BLOCK,
		//ASTC_10X10_SRGB_BLOCK,
		//ASTC_12X10_UNORM_BLOCK,
		//ASTC_12X10_SRGB_BLOCK,
		//ASTC_12X12_UNORM_BLOCK,
		//ASTC_12X12_SRGB_BLOCK,
		_ => {
			//panic!("Format not supported")
			Format::UNDEFINED
		}
	}
}

pub fn stencilable(format: Format) -> bool {
	match format {
		Format::S8_UINT => true,
		Format::D16_UNORM_S8_UINT => true,
		Format::D24_UNORM_S8_UINT => true,
		Format::D32_SFLOAT_S8_UINT => true,
		_ => false,
	}
}
