use crate::engine::{
	graphics::font,
	math::{self, Vector},
	utility::AnyError,
};
use std::path::{Path, PathBuf};

pub struct SDFBuilder {
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

impl Default for SDFBuilder {
	fn default() -> Self {
		SDFBuilder {
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

impl SDFBuilder {
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
	pub fn build(self, font_library: &freetype::Library) -> Result<font::SDF, AnyError> {
		use freetype::{face::LoadFlag, outline::Curve};
		assert!(self.font_path.exists());
		optick::event!("build-font-sdf");
		optick::tag!(
			"font-name",
			self.font_path.file_name().unwrap().to_str().unwrap()
		);
		optick::tag!("glyph-height", self.glyph_height);
		optick::tag!("field-spread", self.field_spread as u32);
		optick::tag!("padding.left", self.padding_per_char.x() as u32);
		optick::tag!("padding.right", self.padding_per_char.y() as u32);
		optick::tag!("padding.top", self.padding_per_char.z() as u32);
		optick::tag!("padding.bottom", self.padding_per_char.w() as u32);
		optick::tag!("min-atlas-size.x", self.minimum_atlas_size.x() as u32);
		optick::tag!("min-atlas-size.y", self.minimum_atlas_size.y() as u32);
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
			optick::event!("calc-sdf");
			optick::tag!("code", char_code as u32);
			face.load_char(char_code, LoadFlag::empty())?;
			// https://www.freetype.org/freetype2/docs/glyphs/glyphs-3.html
			let metrics = face.glyph().metrics();
			let outline = face.glyph().outline().unwrap();

			let metric_size = Vector::new([
				(metrics.width as usize) / 64,
				(metrics.height as usize) / 64,
			]);
			optick::tag!("size.x", metric_size.x() as u32);
			optick::tag!("size.y", metric_size.y() as u32);
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
	fn binpack_pow2_atlas(&self, sorted_fields: &Vec<SDFGlyph>) -> font::SDF {
		optick::event!();
		optick::tag!("glyph-count", sorted_fields.len() as u32);
		optick::tag!("min-atlas-size.x", self.minimum_atlas_size.x() as u32);
		optick::tag!("min-atlas-size.y", self.minimum_atlas_size.y() as u32);
		let mut atlas_size: Vector<usize, 2> = self.minimum_atlas_size;
		loop {
			match SDFBuilder::bin_pack(
				sorted_fields,
				atlas_size.clone(),
				self.padding_per_char.clone(),
			) {
				Some((binary, mut glyphs)) => {
					glyphs.sort_unstable_by(|a, b| a.ascii_code.cmp(&b.ascii_code));
					return font::SDF {
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
		sorted_glyphs: &Vec<SDFGlyph>,
		atlas_size: Vector<usize, 2>,
		padding_lrtb: Vector<usize, 4>, // padding on the left, right, top, and bottom
	) -> Option<(
		/*binary*/ Vec<Vec<Option<u8>>>,
		/*glyphs*/ Vec<font::Glyph>,
	)> {
		optick::event!();
		optick::tag!("atlas-size.x", atlas_size.x() as u32);
		optick::tag!("atlas-size.y", atlas_size.y() as u32);
		// the binary of a grayscale/alpha-only 2D image,
		// where unpopulated "pixels" are represented by `None`.
		let mut atlas_binary: Vec<Vec<Option<u8>>> =
			vec![vec![None; atlas_size.x()]; atlas_size.y()];
		let mut glyphs: Vec<font::Glyph> = Vec::new();

		let padding_on_axis = Vector::new([
			/*x-axis padding*/ padding_lrtb.subvec::<2>(None).total(),
			/*y-axis padding*/ padding_lrtb.subvec::<2>(Some(2)).total(),
		]);
		let padding_offset = Vector::new([padding_lrtb.x(), padding_lrtb.z()]);

		let min_glyph_target_size = sorted_glyphs
			.iter()
			.map(|glyph| glyph.texture_size + padding_on_axis)
			.fold(Vector::new([usize::MAX; 2]), |acc, size| {
				Vector::new([acc.x().min(size.x()), acc.y().min(size.y())])
			});
		let mut empty_cells_in_row: Vec<Vec<std::ops::Range<usize>>> =
			vec![vec![0..atlas_size.x()]; atlas_size.y()];

		// Attempt to place all fields in the atlas
		'place_next_glyph: for (_glyph_idx, glyph) in sorted_glyphs.iter().enumerate() {
			optick::event!("place");
			optick::tag!("glyph", glyph.ascii_code as u32);
			optick::tag!("glyph-size.x", glyph.texture_size.x() as u32);
			optick::tag!("glyph-size.y", glyph.texture_size.y() as u32);

			let glyph_target_size = padding_on_axis + glyph.texture_size;
			for atlas_y in 0..(atlas_size.y() - glyph_target_size.y()) {
				let mut glyph_pos_in_atlas: Option<(Vector<usize, 2>, usize)> = None;
				'place_glyph_in_a_row: for (idx, cell_range) in empty_cells_in_row[atlas_y]
					.iter()
					.filter(|range| (range.end - range.start) >= glyph_target_size.x())
					.enumerate()
				{
					'place_in_cell: for atlas_x in
						cell_range.start..(cell_range.end - glyph_target_size.x())
					{
						let atlas_pos = Vector::new([atlas_x, atlas_y]);
						// If there is already a value inside this cell, then the texel cannot fit and we must search the next cell.
						for target_pos in glyph_target_size.iter(1) {
							let atlas_dest = atlas_pos + target_pos;
							let texel = atlas_binary[atlas_dest.y()][atlas_dest.x()];
							if texel.is_some() {
								continue 'place_in_cell;
							}
						}

						glyph_pos_in_atlas = Some((atlas_pos, idx));
						break 'place_glyph_in_a_row;
					}
				}

				if let Some((atlas_pos, cell_range_idx)) = glyph_pos_in_atlas {
					// Copy the glyph into the atlas
					for glyph_pos in glyph.texture_size.iter(1) {
						let texel = glyph.texels[glyph_pos.y()][glyph_pos.x()];
						let atlas_dest = atlas_pos + glyph_pos + padding_offset;
						atlas_binary[atlas_dest.y()][atlas_dest.x()] = Some(texel);
					}

					// Record the glyph into the lookup metadata
					glyphs.push(font::Glyph {
						ascii_code: glyph.ascii_code,
						atlas_pos: atlas_pos + padding_offset,
						atlas_size: glyph.texture_size,
						metric_size: glyph.metric_size,
						metric_bearing: glyph.metric_bearing,
						metric_advance: glyph.metric_advance,
					});

					// Adjust the atlas metadata so we can continue to use the cell-range optimization
					let cell_range = empty_cells_in_row[atlas_y].remove(cell_range_idx);
					let pre_glyph_range = cell_range.start..atlas_pos.x();
					let post_glyph_range = (atlas_pos.x() + glyph_target_size.x())..cell_range.end;
					// Insert the post-range first so the cell_range_idx can be used as is for both ranges
					if !post_glyph_range.is_empty()
						&& (post_glyph_range.end - post_glyph_range.start) >= min_glyph_target_size.x()
					{
						empty_cells_in_row[atlas_y].insert(cell_range_idx, post_glyph_range);
					}
					if !pre_glyph_range.is_empty()
						&& (pre_glyph_range.end - pre_glyph_range.start) >= min_glyph_target_size.x()
					{
						empty_cells_in_row[atlas_y].insert(cell_range_idx, pre_glyph_range);
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
