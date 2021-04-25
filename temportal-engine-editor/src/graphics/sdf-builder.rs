use crate::{
	engine::{
		math::Vector,
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

									min_dist = min_dist.min(FontSDFBuilder::find_dist_to_line(
										point,
										curve_start,
										curve_end,
									));
									total_cross_count += FontSDFBuilder::find_cross_num_of_line(
										point,
										curve_start,
										curve_end,
									);

									curve_start = curve_end;
								}
								Curve::Bezier2(ctrl, end) => {
									let control = to_f64_vector(ctrl);
									let curve_end = to_f64_vector(end);

									min_dist =
										min_dist.min(FontSDFBuilder::find_dist_to_bezier(
											point,
											curve_start,
											control,
											curve_end,
										));
									total_cross_count +=
										FontSDFBuilder::find_cross_num_of_bezier(
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

	// Based on https://dev.to/thatkyleburke/generating-signed-distance-fields-from-truetype-fonts-calculating-the-distance-33io
	fn find_dist_to_line(
		point: Vector<f64, 2>,
		line_start: Vector<f64, 2>,
		line_end: Vector<f64, 2>,
	) -> f64 {
		// Project the `point` onto the line segment formed by `line_start` and `line_end`.
		// The length of the projected vector is the distance from the point to the line.
		// https://dev-to-uploads.s3.amazonaws.com/i/dkggpi3kk1co139fxpey.png

		// A vector whose direction is the orientation from `line_start` to `line_end`,
		// and whose magnitude is the distance between start and end.
		let line = line_end - line_start;
		// The vector from `line_start` to `point`.
		let start_to_point = point - line_start;

		let dot = Vector::dot(&start_to_point, &line);
		// How far along the line the point `p` is projected to.
		let t = (dot / line.magnitude_sq()).max(0.0).min(1.0);

		// A vector whose direction matches `line`,
		// but whose magnitude is the distance from `start` to the spot `point` is projected to.
		let projection = (line * t) + line_start;
		let point_to_projection = projection - point;

		point_to_projection.magnitude()
	}

	// Based on https://dev.to/thatkyleburke/generating-signed-distance-fields-from-truetype-fonts-calculating-the-distance-33io
	fn find_dist_to_bezier(
		point: Vector<f64, 2>,
		start: Vector<f64, 2>,
		control: Vector<f64, 2>,
		end: Vector<f64, 2>,
	) -> f64 {
		let start_to_control = control - start;
		let point_to_start = start - point;

		// Coefficients of the cubic polynomial
		let a = start - (control * 2.0) + end;
		let d4 = a.magnitude_sq();
		let d3 = Vector::dot(&a, &start_to_control);
		let d2 = Vector::dot(&a, &point_to_start) + (start_to_control.magnitude_sq() * 2.0);
		let d1 = Vector::dot(&start_to_control, &point_to_start);
		let d0 = point_to_start.magnitude_sq();

		// Coefficients of the depressed cubic
		let dp = (d4 * d2 - 3.0 * d3 * d3) / d4.powi(2);
		let dq = (2.0 * d3.powi(3) - (d4 * d3 * d2) + d4.powi(2) * d1) / d4.powi(3);

		// Roots of the depressed cubic
		let discriminant = 4.0 * dp.powi(3) + 27.0 * dq.powi(2);
		let depressed_roots = if dp == 0.0 && dq == 0.0 {
			// 0 is the only solution
			vec![0.0]
		} else if discriminant > 0.0 {
			// only 1 solution, and can use Cardano's formula
			let a = -dq / 2.0;
			let b = (discriminant / 108.0).sqrt();
			vec![(a + b).cbrt() + (a - b).cbrt()]
		} else if discriminant < 0.0 {
			// there are 3 solutions, solvable via trigonometry
			let a = 2.0 * (-dp / 3.0).sqrt();
			let b = (1.0 / 3.0) * ((3.0 * dq) / (2.0 * dp) * (-3.0 / dp).sqrt()).acos();
			(0..3)
				.into_iter()
				.map(|k| a * (b - (2.0 * std::f64::consts::PI * (k as f64) / 3.0)).cos())
				.collect()
		} else {
			let a = 3.0 * dq;
			vec![a / dp, -a / (2.0 * dp)]
		};

		// finally, the minimum distance can be determined based on the roots
		let mut min_dist = f64::MAX;
		for root in depressed_roots {
			let t = (root - d3 / d4).max(0.0).min(1.0);
			let dist = ((d4 * t.powi(4))
				+ (4.0 * d3 * t.powi(3))
				+ (2.0 * d2 * t.powi(2))
				+ (4.0 * d1 * t) + d0)
				.sqrt();
			min_dist = min_dist.min(dist);
		}

		min_dist
	}

	fn find_cross_num_of_line(
		point: Vector<f64, 2>,
		line_start: Vector<f64, 2>,
		line_end: Vector<f64, 2>,
	) -> u32 {
		let line = line_end - line_start;
		// if the line is horizontal, then it is ignored
		if line.y() == 0.0 {
			return 0;
		}

		let t = (point - line_start).y() / line.y();
		let x = line_start.x() + t * line.x();

		let is_right_of_line = x > point.x();
		let between_endpoints = 0.0 < t && t < 1.0;
		let starts_upwards_line = t == 0.0 && line.y().is_sign_positive();
		let ends_downwards_line = t == 1.0 && line.y().is_sign_negative();

		(is_right_of_line && (between_endpoints || starts_upwards_line || ends_downwards_line))
			as u32
	}

	fn find_cross_num_of_bezier(
		point: Vector<f64, 2>,
		start: Vector<f64, 2>,
		control: Vector<f64, 2>,
		end: Vector<f64, 2>,
	) -> u32 {
		let u = start.y() - 2.0 * control.y() + end.y();

		if u == 0.0 {
			let line = end - start;
			let start_to_point = point - start;
			let t = start_to_point.y() / line.y();
			let a = 1.0 - t;
			let x = (a.powi(2) * start.x()) + (2.0 * a * t * control.x()) + (t.powi(2) * end.x());

			let is_right_of_line = x > point.x();
			let between_endpoints = 0.0 < t && t < 1.0;
			let starts_upwards_line = t == 0.0 && line.y().is_sign_positive();
			let ends_downwards_line = t == 1.0 && line.y().is_sign_negative();

			return (is_right_of_line
				&& (between_endpoints || starts_upwards_line || ends_downwards_line)) as u32;
		}

		let w = (point.y() * start.y()) - (2.0 * point.y() * control.y()) + (point.y() * end.y())
			- (start.y() * end.y())
			+ control.y().powi(2);

		if w.is_sign_negative() {
			return 0;
		}

		let w = w.sqrt();
		let control_to_start = start - control;

		let intercept = |t: f64| -> f64 {
			let a = 1.0 - t;
			(a.powi(2) * start.x()) + (2.0 * a * t * control.x()) + (t.powi(2) * end.x())
		};

		let t1 = (control_to_start.y() + w) / u;
		let x1 = intercept(t1);
		let t2 = (control_to_start.y() - w) / u;
		let x2 = intercept(t2);
		let start_dir = if start.y() == control.y() {
			end - start
		} else {
			control - start
		};
		let end_dir = if end.y() == control.y() {
			end - start
		} else {
			end - control
		};

		if t1 == t2 {
			let is_right_of_line = x1 > point.x();
			let starts_upwards_line = t1 == 0.0 && start_dir.y().is_sign_positive();
			let ends_downwards_line = t1 == 1.0 && end_dir.y().is_sign_negative();
			(is_right_of_line && (starts_upwards_line || ends_downwards_line)) as u32
		} else {
			let is_crossing = |x: f64, t: f64| -> bool {
				let is_right_of_line = x > point.x();
				let between_endpoints = 0.0 < t && t < 1.0;
				let starts_upwards_line = t == 0.0 && start_dir.y().is_sign_positive();
				let ends_downwards_line = t == 1.0 && end_dir.y().is_sign_negative();
				is_right_of_line
					&& (between_endpoints || starts_upwards_line || ends_downwards_line)
			};

			(is_crossing(x1, t1) as u32) + (is_crossing(x2, t2) as u32)
		}
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

