use crate::{
	engine::{
		math::{self, Vector},
		utility::AnyError,
	}
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
			minimum_atlas_size: Vector::new([ 256; 2 ]),
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
	pub fn build(
		self,
		font_library: &freetype::Library,
	) -> Result<(Vector<usize, 2>, Vec<Vec<u8>>), AnyError> {
		use freetype::{face::LoadFlag, outline::Curve};
		assert!(self.font_path.exists());
		log::debug!(
			"Creating SDF for font {:?}",
			self.font_path.file_name().unwrap()
		);

		let face = font_library.new_face(self.font_path, 0)?;
		face.set_pixel_sizes(0, self.glyph_height)?;

		let to_f64_vector =
			|v: freetype::Vector| Vector::new([(v.x as f64) / 64.0, (v.y as f64) / 64.0]);
		let spread_f = self.field_spread as f64;

		let mut signed_dist_fields = Vec::new();

		for char_code in self.char_range.clone() {
			face.load_char(char_code, LoadFlag::empty())?;
			let metrics = face.glyph().metrics();
			let metrics_size = Vector::new([
				((metrics.width as usize) / 64) + (self.field_spread * 2),
				((metrics.height as usize) / 64) + (self.field_spread * 2),
			]);
			let left_edge_padded = ((metrics.horiBearingX as f64) / 64.0) - spread_f;
			let top_edge_padded = ((metrics.horiBearingY as f64) / 64.0) + spread_f;
			let outline = face.glyph().outline().unwrap();

			let mut texels_in_glyph = Vec::with_capacity(metrics_size.y());

			for row in 0..metrics_size.y() {
				let mut texel_line = Vec::with_capacity(metrics_size.x());

				for column in 0..metrics_size.x() {
					let mut min_dist = f64::MAX;
					let mut total_cross_count = 0;
					let point = Vector::new([
						left_edge_padded + (column as f64) + 0.5,
						top_edge_padded - (row as f64) - 0.5,
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

									min_dist =
										min_dist.min(math::ops::distance_point_to_bezier(
											point,
											curve_start,
											control,
											curve_end,
										));
									total_cross_count +=
										math::ops::count_intercepts_on_bezier(
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

					texel_line.push(dist_scaled as u8);
				}

				texels_in_glyph.push(texel_line);
			}

			signed_dist_fields.push(texels_in_glyph);
		}

		signed_dist_fields
			.sort_unstable_by(|a, b| (b.len() * b[0].len()).cmp(&(a.len() * a[0].len())));
		log::debug!("SDF calculations complete, starting atlas generation.");

		let mut atlas_size: Vector<usize, 2> = self.minimum_atlas_size;
		let atlas_binary = FontSDFBuilder::bin_pack_expanding_pow2(
			&signed_dist_fields,
			&mut atlas_size,
			&self.padding_per_char,
		);
		log::debug!(
			"Packed {} glyphs into a {} SDF texture",
			signed_dist_fields.len(),
			atlas_size
		);

		Ok((
			atlas_size,
			atlas_binary
				.into_iter()
				.map(|texel_line| {
					texel_line
						.into_iter()
						.map(|alpha| alpha.unwrap_or(0))
						.collect()
				})
				.collect(),
		))
	}

	/// Pack an unknown nuimber of rectangles into a single rectangle with the smallest possible size.
	/// Not the optimal solution, but a reasonable one that is more performant - O(nlog(n)).
	/// This specific bin-packing ensures that the final solution is a rectangle whose sides are both powers of 2,
	/// which is important for platforms which need to maximize texture compressions. This requirement turns the
	/// original performance of O(nlog(n)) into O(mnlog(n)) where m is the number of iterations required when resizing the atlas.
	/// https://en.wikipedia.org/wiki/Bin_packing_problem#First_Fit_Decreasing_(FFD)
	/// https://dev.to/thatkyleburke/generating-signed-distance-fields-from-truetype-fonts-generating-the-texture-atlas-1l0
	fn bin_pack_expanding_pow2(
		sorted_fields: &Vec</*Texel*/ Vec<Vec<u8>>>,
		atlas_size: &mut Vector<usize, 2>,
		padding_per_field: &Vector<usize, 4>,
	) -> Vec<Vec<Option<u8>>> {
		loop {
			match FontSDFBuilder::bin_pack(
				sorted_fields,
				atlas_size.clone(),
				padding_per_field.clone(),
			) {
				Some(atlas_binary) => return atlas_binary,
				// Bin packing failed, expand in the dimension that is smallest
				None => {
					// Expand the atlas such that each dimension that is being expanded (width and/or height),
					// the atlas size doubles (maintaining power of 2).
					let dimensions_to_expand_in = Vector::new([
						(atlas_size.x() == atlas_size.y() || atlas_size.x() < atlas_size.y())
							as usize,
						(atlas_size.y() < atlas_size.x()) as usize,
					]);
					*atlas_size += dimensions_to_expand_in * (*atlas_size);
				}
			}
		}
	}

	fn bin_pack(
		// each glyph is a 2D array of alpha texels
		sorted_glyps: &Vec</*glyph*/ Vec<Vec<u8>>>,
		atlas_size: Vector<usize, 2>,
		padding_lrtb: Vector<usize, 4>, // padding on the left, right, top, and bottom
	) -> Option<Vec<Vec<Option<u8>>>> {
		// the binary of a grayscale/alpha-only 2D image,
		// where unpopulated "pixels" are represented by `None`.
		let mut atlas_binary: Vec<Vec<Option<u8>>> =
			vec![vec![None; atlas_size.x()]; atlas_size.y()];
		let padding_on_axis = Vector::new([
			/*x-axis padding*/ padding_lrtb.subvec::<2>(None).total(),
			/*y-axis padding*/ padding_lrtb.subvec::<2>(Some(2)).total(),
		]);
		let padding_offset = Vector::new([padding_lrtb.x(), padding_lrtb.z()]);
		// Attempt to place all fields in the atlas
		'place_next_glyph: for (_glyph_idx, glyph_texels) in sorted_glyps.iter().enumerate() {
			// This is then width and height of a given texel
			let texel_count = [glyph_texels[0].len() as usize, glyph_texels.len() as usize];
			let glyph_size = Vector::new(texel_count);
			let glyph_target_size = padding_on_axis + glyph_size;
			// The size of the atlas that can be iterated over while still leaving enough room for the glyph itself.
			let leading_size = atlas_size - glyph_target_size;
			for atlas_y in 0..leading_size.y() {
				'place_in_next_column: for atlas_x in 0..leading_size.x() {
					let atlas_pos = Vector::new([atlas_x, atlas_y]);
					// If there is already a value inside this cell, then the texel cannot fit and we must search the next cell.
					for target_y in 0..glyph_target_size.y() {
						for target_x in 0..glyph_target_size.x() {
							let target_pos = Vector::new([target_x, target_y]);
							let atlas_dest = atlas_pos + target_pos;
							let texel = atlas_binary[atlas_dest.y()][atlas_dest.x()];
							if texel.is_some() {
								continue 'place_in_next_column;
							}
						}
					}
					// Since there are no other pixels in the desired area,
					// write the pixels of the texel into the atlas pixels.
					for glyph_y in 0..glyph_size.y() {
						for glyph_x in 0..glyph_size.x() {
							let texel = glyph_texels[glyph_y][glyph_x];
							let glyph_pos = Vector::new([glyph_x, glyph_y]);
							let atlas_dest = atlas_pos + glyph_pos + padding_offset;
							atlas_binary[atlas_dest.y()][atlas_dest.x()] = Some(texel);
						}
					}
					continue 'place_next_glyph;
				}
			}
			// The texel was not placed (otherwise `field_loop` would have continued).
			// This means the atlas was not big enough to fix all the texels.
			return None;
		}
		// Convert a valid packing such that all unused pixels are completely transparent
		// (where each pixel is just alpha, no rgb).
		Some(atlas_binary)
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

