use crate::{
	asset::{self, AssetResult, TypeMetadata},
	math::Vector,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Font {
	asset_type: String,
	sdf: Option<SDF>,
}

/// https://www.freetype.org/freetype2/docs/glyphs/glyphs-2.html
/// https://www.freetype.org/freetype2/docs/glyphs/glyphs-3.html
/// https://freetype.org/freetype2/docs/reference/ft2-base_interface.html#ft_facerec
#[derive(Serialize, Deserialize, Clone)]
pub struct SDF {
	pub size: Vector<usize, 2>,
	pub binary: Vec<Vec<u8>>,
	pub glyphs: Vec<Glyph>,
	/// The y-distance from the previous baseline to the next.
	/// Derived from the `Scaled Global Metrics` https://www.freetype.org/freetype2/docs/tutorial/step2.html#section-3
	/// Expressed as a scalar value to be multiplied by the font size.
	pub line_height: f32,
}

impl std::fmt::Debug for SDF {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"SDL {{ size:{:?}, glyphs:{:?}, binary:(omitted) }}",
			self.size,
			self.glyphs.len()
		)
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Glyph {
	pub unicode: u32,
	// The position (in pixels) of the glyph in the atlas texture.
	pub atlas_pos: Vector<usize, 2>,
	// The size (in pixels) of the glyph in the atlas texture.
	pub atlas_size: Vector<usize, 2>,
	pub metrics: GlyphMetrics,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct GlyphMetrics {
	/// The size of the glyph in metrics.
	/// Expressed as a scalar value to be multiplied by the font size.
	pub size: Vector<f32, 2>,
	/// The bearing meteric, where x is the distance to the right of the pen position,
	/// and y is the distance up from the pen position,
	/// that the glyph's top-left position should be rendered at.
	/// Expressed as a scalar value to be multiplied by the font size.
	pub bearing: Vector<f32, 2>,
	/// The amount of space to the right that the pen position should move,
	/// when the glyph is rendered.
	/// Expressed as a scalar value to be multiplied by the font size.
	pub advance: f32,
}

impl Glyph {
	pub fn as_char(&self) -> char {
		char::from_u32(self.unicode as u32).unwrap()
	}
}

impl asset::Asset for Font {
	fn metadata() -> Box<dyn TypeMetadata> {
		Box::new(FontMetadata {})
	}
}

impl Font {
	pub fn set_sdf(&mut self, sdf: SDF) {
		self.sdf = Some(sdf);
	}

	pub fn glyphs(&self) -> &Vec<Glyph> {
		&self.sdf.as_ref().unwrap().glyphs
	}

	pub fn binary(&self) -> &Vec<Vec<u8>> {
		&self.sdf.as_ref().unwrap().binary
	}

	pub fn size(&self) -> &Vector<usize, 2> {
		&self.sdf.as_ref().unwrap().size
	}

	pub fn line_height(&self) -> &f32 {
		&self.sdf.as_ref().unwrap().line_height
	}
}

pub struct FontMetadata {}

impl TypeMetadata for FontMetadata {
	fn name(&self) -> asset::TypeId {
		"font"
	}

	fn decompile(&self, bin: &Vec<u8>) -> AssetResult {
		let shader: Font = rmp_serde::from_read_ref(&bin)?;
		Ok(Box::new(shader))
	}
}

impl std::ops::Mul</*font_size*/ f32> for GlyphMetrics {
	type Output = GlyphMetrics;
	fn mul(self, font_size: f32) -> Self::Output {
		Self {
			size: self.size * font_size,
			bearing: self.bearing * font_size,
			advance: self.advance * font_size,
		}
	}
}
