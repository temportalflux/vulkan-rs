use crate::engine::{
	math::{self, Vector},
	utility::AnyError,
	graphics::Glyph
};
use std::path::{Path, PathBuf};

pub struct FontSDFBuilder {
	font_path: PathBuf,
	char_range: std::ops::RangeInclusive<usize>,
	glyph_height: u32,
	field_spread: usize,
	padding_per_char: Vector<usize, 4>,
	minimum_atlas_size: Vector<usize, 2>,
}

struct SDFGlyph {
	ascii_code: usize,
	texture_size: Vector<usize, 2>,
	texels: Vec<Vec<u8>>,
	metric_size: Vector<usize, 2>,
	metric_bearing: Vector<usize, 2>,
	metric_advance: usize,
}

pub struct FontSDF {
	pub size: Vector<usize, 2>,
	pub binary: Vec<Vec<u8>>,
	pub glyphs: Vec<Glyph>,
}

impl Default for FontSDFBuilder {
	fn default() -> FontSDFBuilder {
		FontSDFBuilder {
			font_path: PathBuf::default(),
			// only ASCII is supported atm, and 33-126 is the inclusive range of all printable ASCII characters
			// http://facweb.cs.depaul.edu/sjost/it212/documents/ascii-pr.htm
			char_range: 33..=126,
			glyph_height: 10,
			field_spread: 10,
			padding_per_char: Vector::new([0; 4]),
			minimum_atlas_size: Vector::new([256; 2]),
		}
	}
}

impl FontSDFBuilder {
	pub fn with_font_path(mut self, path: &Path) -> Self {
		self.font_path = path.to_path_buf();
		self
	}

	pub fn with_glyph_height(mut self, height: u32) -> Self {
		self.glyph_height = height;
		self
	}

	pub fn with_spread(mut self, spread: usize) -> Self {
		self.field_spread = spread;
		self
	}

	pub fn with_padding(mut self, padding: Vector<usize, 4>) -> Self {
		self.padding_per_char = padding;
		self
	}

	pub fn with_minimum_atlas_size(mut self, size: Vector<usize, 2>) -> Self {
		self.minimum_atlas_size = size;
		self
	}

	/// Compiles the font ttf at `path` into a signed-distance-field.
	/// Algorithm based on https://dev.to/thatkyleburke/generating-signed-distance-fields-from-truetype-fonts-introduction-code-setup-25lh.
	pub fn build(self, font_library: &freetype::Library) -> Result<FontSDF, AnyError> {
		use freetype::{face::LoadFlag, outline::Curve};
		assert!(self.font_path.exists());
		log::debug!(
			"Creating SDF for font {:?}",
			self.font_path.file_name().unwrap()
		);

		let face = font_library.new_face(&self.font_path, 0)?;
		face.set_pixel_sizes(0, self.glyph_height)?;

		let to_f64_vector =
			|v: freetype::Vector| Vector::new([(v.x as f64) / 64.0, (v.y as f64) / 64.0]);
		let spread_f = self.field_spread as f64;
		let spread_size = Vector::new([self.field_spread * 2; 2]);

		let mut glyphs: Vec<SDFGlyph> = Vec::new();

		for char_code in self.char_range.clone() {
			face.load_char(char_code, LoadFlag::empty())?;
			// https://www.freetype.org/freetype2/docs/glyphs/glyphs-3.html
			let metrics = face.glyph().metrics();
			let outline = face.glyph().outline().unwrap();

			let metric_size = Vector::new([
				(metrics.width as usize) / 64,
				(metrics.height as usize) / 64,
			]);
			let metric_advance = (metrics.horiAdvance as usize) / 64;
			let metric_bearing = Vector::new([
				(metrics.horiBearingX as usize) / 64,
				(metrics.horiBearingY as usize) / 64,
			]);

			let texture_size = metric_size + spread_size;
			let left_edge_padded = metric_bearing.x() as f64 - spread_f;
			let top_edge_padded = metric_bearing.y() as f64 + spread_f;

			let mut texels: Vec<Vec<u8>> = vec![vec![0; texture_size.x()]; texture_size.y()];

			for glyph_pos in texture_size.iter(1) {
				let mut min_dist = f64::MAX;
				let mut total_cross_count = 0;
				let point = Vector::new([
					left_edge_padded + (glyph_pos.x() as f64) + 0.5,
					top_edge_padded - (glyph_pos.y() as f64) - 0.5,
				]);

				for contour in outline.contours_iter() {
					let start = *contour.start();
					let mut curve_start = to_f64_vector(start);

					for curve in contour {
						match curve {
							Curve::Line(end) => {
								let curve_end = to_f64_vector(end);

								min_dist = min_dist.min(math::ops::distance_point_to_line_segment(
									point,
									curve_start,
									curve_end,
								));
								total_cross_count += math::ops::has_crossed_line_segment(
									point,
									curve_start,
									curve_end,
								) as u32;

								curve_start = curve_end;
							}
							Curve::Bezier2(ctrl, end) => {
								let control = to_f64_vector(ctrl);
								let curve_end = to_f64_vector(end);

								min_dist = min_dist.min(math::ops::distance_point_to_bezier(
									point,
									curve_start,
									control,
									curve_end,
								));
								total_cross_count += math::ops::count_intercepts_on_bezier(
									point,
									curve_start,
									control,
									curve_end,
								);

								curve_start = curve_end;
							}
							Curve::Bezier3(_, _, _) => {
								return Err(Box::new(FontError::CubicBezier()));
							}
						};
					}
				}

				let dist_signed =
					(((total_cross_count % 2 == 0) as u32) as f64 * -2.0 + 1.0) * min_dist;
				let dist_clamped_to_spread = dist_signed.min(spread_f).max(-spread_f);
				let dist_zero_to_one = (dist_clamped_to_spread + spread_f) / (spread_f * 2.0);
				let dist_scaled = (dist_zero_to_one * 255.0).round();

				texels[glyph_pos.y()][glyph_pos.x()] = dist_scaled as u8;
			}

			glyphs.push(SDFGlyph {
				ascii_code: char_code,
				texture_size,
				texels,
				metric_size,
				metric_bearing,
				metric_advance,
			});
		}

		glyphs.sort_unstable_by(|a, b| {
			(b.texture_size.y() * b.texture_size.x())
				.cmp(&(a.texture_size.y() * a.texture_size.x()))
		});
		log::debug!("SDF calculations complete, starting atlas generation.");

		let font_sdf = self.binpack_pow2_atlas(&glyphs);

		log::debug!(
			"Packed {} glyphs into a {} SDF texture",
			font_sdf.glyphs.len(),
			font_sdf.size
		);

		Ok(font_sdf)
	}

	/// Pack an unknown nuimber of rectangles into a single rectangle with the smallest possible size.
	/// Not the optimal solution, but a reasonable one that is more performant - O(nlog(n)).
	/// This specific bin-packing ensures that the final solution is a rectangle whose sides are both powers of 2,
	/// which is important for platforms which need to maximize texture compressions. This requirement turns the
	/// original performance of O(nlog(n)) into O(mnlog(n)) where m is the number of iterations required when resizing the atlas.
	/// https://en.wikipedia.org/wiki/Bin_packing_problem#First_Fit_Decreasing_(FFD)
	/// https://dev.to/thatkyleburke/generating-signed-distance-fields-from-truetype-fonts-generating-the-texture-atlas-1l0
	fn binpack_pow2_atlas(&self, sorted_fields: &Vec<SDFGlyph>) -> FontSDF {
		let mut atlas_size: Vector<usize, 2> = self.minimum_atlas_size;
		loop {
			match FontSDFBuilder::bin_pack(
				sorted_fields,
				atlas_size.clone(),
				self.padding_per_char.clone(),
			) {
				Some((binary, mut glyphs)) => {
					glyphs.sort_unstable_by(|a, b| a.ascii_code.cmp(&b.ascii_code));
					return FontSDF {
						size: atlas_size,
						binary: binary
							.into_iter()
							.map(|texel_line| {
								texel_line
									.into_iter()
									.map(|alpha| alpha.unwrap_or(0))
									.collect()
							})
							.collect(),
						glyphs,
					};
				}
				// Bin packing failed, expand in the dimension that is smallest
				None => {
					// Expand the atlas such that each dimension that is being expanded (width and/or height),
					// the atlas size doubles (maintaining power of 2).
					let dimensions_to_expand_in = Vector::new([
						(atlas_size.x() == atlas_size.y() || atlas_size.x() < atlas_size.y())
							as usize,
						(atlas_size.y() < atlas_size.x()) as usize,
					]);
					atlas_size += dimensions_to_expand_in * atlas_size;
				}
			}
		}
	}

	fn bin_pack(
		// each glyph is a 2D array of alpha texels
		sorted_glyps: &Vec<SDFGlyph>,
		atlas_size: Vector<usize, 2>,
		padding_lrtb: Vector<usize, 4>, // padding on the left, right, top, and bottom
	) -> Option<(
		/*binary*/ Vec<Vec<Option<u8>>>,
		/*glyphs*/ Vec<Glyph>,
	)> {
		// the binary of a grayscale/alpha-only 2D image,
		// where unpopulated "pixels" are represented by `None`.
		let mut atlas_binary: Vec<Vec<Option<u8>>> =
			vec![vec![None; atlas_size.x()]; atlas_size.y()];
		let mut glyphs: Vec<Glyph> = Vec::new();
		let padding_on_axis = Vector::new([
			/*x-axis padding*/ padding_lrtb.subvec::<2>(None).total(),
			/*y-axis padding*/ padding_lrtb.subvec::<2>(Some(2)).total(),
		]);
		let padding_offset = Vector::new([padding_lrtb.x(), padding_lrtb.z()]);
		// Attempt to place all fields in the atlas
		'place_next_glyph: for (_glyph_idx, glyph) in sorted_glyps.iter().enumerate() {
			// This is then width and height of a given texel
			let glyph_target_size = padding_on_axis + glyph.texture_size;
			// The size of the atlas that can be iterated over while still leaving enough room for the glyph itself.
			let leading_size = atlas_size - glyph_target_size;
			for atlas_y in 0..leading_size.y() {
				'place_in_next_column: for atlas_x in 0..leading_size.x() {
					let atlas_pos = Vector::new([atlas_x, atlas_y]);
					// If there is already a value inside this cell, then the texel cannot fit and we must search the next cell.
					for target_pos in glyph_target_size.iter(1) {
						let atlas_dest = atlas_pos + target_pos;
						let texel = atlas_binary[atlas_dest.y()][atlas_dest.x()];
						if texel.is_some() {
							continue 'place_in_next_column;
						}
					}
					// Since there are no other pixels in the desired area,
					// write the pixels of the texel into the atlas pixels.
					for glyph_pos in glyph.texture_size.iter(1) {
						let texel = glyph.texels[glyph_pos.y()][glyph_pos.x()];
						let atlas_dest = atlas_pos + glyph_pos + padding_offset;
						atlas_binary[atlas_dest.y()][atlas_dest.x()] = Some(texel);
					}
					glyphs.push(Glyph {
						ascii_code: glyph.ascii_code,
						atlas_pos: atlas_pos + padding_offset,
						atlas_size: glyph.texture_size,
						metric_size: glyph.metric_size,
						metric_bearing: glyph.metric_bearing,
						metric_advance: glyph.metric_advance,
					});
					continue 'place_next_glyph;
				}
			}
			// The texel was not placed (otherwise `field_loop` would have continued).
			// This means the atlas was not big enough to fix all the texels.
			return None;
		}
		// Convert a valid packing such that all unused pixels are completely transparent
		// (where each pixel is just alpha, no rgb).
		Some((atlas_binary, glyphs))
	}
}

#[derive(Debug)]
pub enum FontError {
	CubicBezier(),
}

impl std::fmt::Display for FontError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			FontError::CubicBezier() => {
				write!(f, "Cubic bezier curves in font files are not supported")
			}
		}
	}
}

impl std::error::Error for FontError {}
