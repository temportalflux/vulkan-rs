use crate::engine::{
	graphics::font,
	math::{self, vector, Vector},
	profiling::{self, optick},
	utility::AnyError,
};
use std::path::{Path, PathBuf};

pub struct SDFBuilder {
	font_path: PathBuf,
	unicode_ranges: Vec<std::ops::RangeInclusive<u32>>,
	glyph_height: u32,
	field_spread: usize,
	padding_per_char: Vector<usize, 4>,
	minimum_atlas_size: Vector<usize, 2>,
}

struct SDFGlyph {
	unicode: u32,
	texture_size: Vector<usize, 2>,
	texels: Vec<Vec<u8>>,
	metric_size: Vector<f32, 2>,
	metric_bearing: Vector<f32, 2>,
	metric_advance: f32,
}

impl Default for SDFBuilder {
	fn default() -> Self {
		SDFBuilder {
			font_path: PathBuf::default(),
			// https://en.wikipedia.org/wiki/List_of_Unicode_characters#Basic_Latin
			unicode_ranges: vec![
				33..=126,  // basic latin
				161..=172, // latin supplement 1
			],
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
		profiling::scope!("build-font-sdf");
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

		let to_f32_vector =
			|v: freetype::Vector| Vector::new([(v.x as f32) / 64.0, (v.y as f32) / 64.0]);
		let spread_f = self.field_spread as f32;
		let spread_size = Vector::new([self.field_spread * 2; 2]);

		let mut glyphs: Vec<SDFGlyph> = Vec::new();

		{
			profiling::scope!("calc-sdf-all");
			for unicode in self.unicode_ranges.iter().flat_map(|codes| codes.clone()) {
				profiling::scope!("calc-sdf");
				optick::tag!("code", unicode);

				face.load_char(unicode as usize, LoadFlag::empty())?;
				// https://www.freetype.org/freetype2/docs/glyphs/glyphs-3.html
				let metrics = face.glyph().metrics();
				let outline = face.glyph().outline().unwrap();

				let metric_size = Vector::new([
					(metrics.width as usize) / 64,
					(metrics.height as usize) / 64,
				]);
				optick::tag!("size.x", metric_size.x() as u32);
				optick::tag!("size.y", metric_size.y() as u32);
				let metric_bearing = Vector::new([
					(metrics.horiBearingX as usize) / 64,
					(metrics.horiBearingY as usize) / 64,
				]);

				let texture_size = metric_size + spread_size;
				let left_edge_padded = metric_bearing.x() as f32 - spread_f;
				let top_edge_padded = metric_bearing.y() as f32 + spread_f;

				let mut texels: Vec<Vec<u8>> = vec![vec![0; texture_size.x()]; texture_size.y()];

				for glyph_pos in texture_size.iter(1) {
					let mut min_dist = f32::MAX;
					let mut total_cross_count = 0;
					let point = Vector::new([
						left_edge_padded + (glyph_pos.x() as f32) + 0.5,
						top_edge_padded - (glyph_pos.y() as f32) - 0.5,
					]);

					for contour in outline.contours_iter() {
						let start = *contour.start();
						let mut curve_start = to_f32_vector(start);

						for curve in contour {
							match curve {
								Curve::Line(end) => {
									let curve_end = to_f32_vector(end);

									min_dist =
										min_dist.min(math::ops::distance_point_to_line_segment(
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
									let control = to_f32_vector(ctrl);
									let curve_end = to_f32_vector(end);

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
						(((total_cross_count % 2 == 0) as u32) as f32 * -2.0 + 1.0) * min_dist;
					let dist_clamped_to_spread = dist_signed.min(spread_f).max(-spread_f);
					let dist_zero_to_one = (dist_clamped_to_spread + spread_f) / (spread_f * 2.0);
					let dist_scaled = (dist_zero_to_one * 255.0).round();

					texels[glyph_pos.y()][glyph_pos.x()] = dist_scaled as u8;
				}

				let rendered_font_size = self.glyph_height as f32;
				let rescale = 64.0 * rendered_font_size;
				glyphs.push(SDFGlyph {
					unicode,
					texture_size,
					texels,
					metric_size: vector![metrics.width as f32, metrics.height as f32] / rescale,
					metric_bearing: vector![
						metrics.horiBearingX as f32,
						metrics.horiBearingY as f32
					] / rescale,
					metric_advance: metrics.horiAdvance as f32 / rescale,
				});
			}
		}

		glyphs.sort_unstable_by(|a, b| {
			b.texture_size
				.y()
				.cmp(&a.texture_size.y())
				.then(b.texture_size.x().cmp(&a.texture_size.x()))
		});
		log::debug!("SDF calculations complete, starting atlas generation.");

		let (size, binary, glyphs) = self.binpack_pow2_atlas(&glyphs);
		log::debug!("Packed {} glyphs into a {} SDF texture", glyphs.len(), size);

		Ok(font::SDF {
			size,
			binary,
			glyphs,
			line_height: face.size_metrics().unwrap().height as f32
				/ (72.0 * (self.glyph_height as f32)),
		})
	}

	/// Pack an unknown nuimber of rectangles into a single rectangle with the smallest possible size.
	/// Not the optimal solution, but a reasonable one that is more performant - O(nlog(n)).
	/// This specific bin-packing ensures that the final solution is a rectangle whose sides are both powers of 2,
	/// which is important for platforms which need to maximize texture compressions. This requirement turns the
	/// original performance of O(nlog(n)) into O(mnlog(n)) where m is the number of iterations required when resizing the atlas.
	/// https://en.wikipedia.org/wiki/Bin_packing_problem#First_Fit_Decreasing_(FFD)
	/// https://dev.to/thatkyleburke/generating-signed-distance-fields-from-truetype-fonts-generating-the-texture-atlas-1l0
	#[profiling::function]
	fn binpack_pow2_atlas(
		&self,
		sorted_fields: &Vec<SDFGlyph>,
	) -> (Vector<usize, 2>, Vec<Vec<u8>>, Vec<font::Glyph>) {
		optick::tag!("glyph-count", sorted_fields.len() as u32);
		optick::tag!("min-atlas-size.x", self.minimum_atlas_size.x() as u32);
		optick::tag!("min-atlas-size.y", self.minimum_atlas_size.y() as u32);
		let mut atlas_size: Vector<usize, 2> = self.minimum_atlas_size;
		let padding_offset = Vector::new([self.padding_per_char.x(), self.padding_per_char.z()]);
		let padding_on_axis = Vector::new([
			/*x-axis padding*/ self.padding_per_char.subvec::<2>(None).total(),
			/*y-axis padding*/ self.padding_per_char.subvec::<2>(Some(2)).total(),
		]);
		let glyph_sizes = sorted_fields
			.iter()
			.map(|glyph| glyph.texture_size + padding_on_axis)
			.collect::<Vec<_>>();
		loop {
			match SDFBuilder::plan_cell_packing(atlas_size.clone(), &glyph_sizes) {
				Some(glyph_positions) => {
					profiling::scope!("process-packing");
					let mut binary: Vec<Vec<u8>> = vec![vec![0; atlas_size.x()]; atlas_size.y()];
					let mut glyphs: Vec<font::Glyph> = Vec::new();
					for (field, &atlas_pos) in sorted_fields.iter().zip(glyph_positions.iter()) {
						// Copy the glyph into the atlas
						for glyph_pos in field.texture_size.iter(1) {
							let texel = field.texels[glyph_pos.y()][glyph_pos.x()];
							let atlas_dest = atlas_pos + glyph_pos + padding_offset;
							binary[atlas_dest.y()][atlas_dest.x()] = texel;
						}

						glyphs.push(font::Glyph {
							unicode: field.unicode,
							atlas_pos: atlas_pos + padding_offset,
							atlas_size: field.texture_size,
							metrics: font::GlyphMetrics {
								size: field.metric_size,
								bearing: field.metric_bearing,
								advance: field.metric_advance,
							},
						});
					}
					glyphs.sort_unstable_by(|a, b| a.unicode.cmp(&b.unicode));
					return (atlas_size, binary, glyphs);
				}
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

	/// Packs a list of item sizes into a 2D grid.
	/// Assumes that `cell_item_sizes` are sorted in decreasing height,
	/// and when heights match, in decreasing width.
	/// Packs cell items starting in the top left of the cell-space defined by `atlas_size`,
	/// and moving first across the row and then to the next row.
	/// If some value is returned, it will always have the same length as `cell_item_sizes`,
	/// where each returned position matches 1:1 with a provided size at the same index.
	#[profiling::function]
	fn plan_cell_packing(
		atlas_size: Vector<usize, 2>,
		cell_item_sizes: &Vec</*size*/ Vector<usize, 2>>,
	) -> Option<Vec</*position*/ Vector<usize, 2>>> {
		optick::tag!("atlas-size.x", atlas_size.x() as u32);
		optick::tag!("atlas-size.y", atlas_size.y() as u32);

		let min_item_size = cell_item_sizes
			.iter()
			.fold(Vector::new([usize::MAX; 2]), |acc, size| {
				Vector::new([acc.x().min(size.x()), acc.y().min(size.y())])
			});

		#[derive(Debug)]
		struct Cell {
			pos: Vector<usize, 2>,
			size: Vector<usize, 2>,
		}

		let mut cells = vec![Cell {
			pos: Vector::filled(0),
			size: atlas_size,
		}];
		let mut cell_item_positions: Vec<Vector<usize, 2>> =
			vec![Vector::filled(0); cell_item_sizes.len()];

		let insert_cell = |list: &mut Vec<Cell>, cell: Cell| {
			if cell.size.x() >= min_item_size.x() && cell.size.y() >= min_item_size.y() {
				match list.binary_search_by(|existing| {
					existing
						.size
						.y()
						.cmp(&cell.size.y())
						.then(existing.size.x().cmp(&cell.size.x()))
						.then(existing.pos.y().cmp(&cell.pos.y()))
						.then(existing.pos.x().cmp(&cell.pos.x()))
				}) {
					Ok(existing_idx) => {
						log::error!(
							"Tried inserting cell that already exists {:?} at {} => {:?}",
							cell,
							existing_idx,
							list[existing_idx]
						);
					}
					Err(insert_sort_idx) => {
						list.insert(insert_sort_idx, cell);
					}
				}
			}
		};

		for (idx_item, (item_size, item_pos_out)) in cell_item_sizes
			.iter()
			.zip(cell_item_positions.iter_mut())
			.enumerate()
		{
			profiling::scope!("pack-item");
			optick::tag!("item", idx_item as u32);
			match cells
				.iter()
				.enumerate()
				.find(|(_, cell)| item_size.x() <= cell.size.x() && item_size.y() <= cell.size.y())
				.map(|(idx_cell, _)| idx_cell)
			{
				Some(idx_cell) => {
					let cell = cells.remove(idx_cell);
					*item_pos_out = cell.pos;
					// A new cell formed by the remainder to the right of the placed item.
					// the remainder below the item is discarded.
					let item_width = Vector::new([item_size.x(), 0]);
					let item_height = Vector::new([0, item_size.y()]);
					insert_cell(
						&mut cells,
						Cell {
							pos: cell.pos + item_width,
							size: Vector::new([cell.size.x() - item_size.x(), item_size.y()]),
						},
					);
					insert_cell(
						&mut cells,
						Cell {
							pos: cell.pos + item_height,
							size: cell.size - item_height,
						},
					);
				}
				None => return None,
			}
		}

		Some(cell_item_positions)
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
