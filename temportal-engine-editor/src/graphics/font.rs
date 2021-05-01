use crate::{
	asset::TypeEditorMetadata,
	engine::{
		asset::{AnyBox, AssetResult},
		graphics::font::Font,
		math::Vector,
		utility::AnyError,
	},
};
use serde_json;
use std::{
	path::{Path, PathBuf},
	time::SystemTime,
};

#[path = "sdf-builder.rs"]
mod sdf_builder;
pub use sdf_builder::*;

pub struct FontEditorMetadata {}

impl FontEditorMetadata {
	pub fn boxed() -> Box<dyn TypeEditorMetadata> {
		Box::new(FontEditorMetadata {})
	}
	fn font_path(&self, asset_path: &Path, is_bold: bool, is_italic: bool) -> PathBuf {
		let mut path = asset_path.parent().unwrap().to_path_buf();
		path.push(asset_path.file_stem().unwrap());
		path.push(match (is_bold, is_italic) {
			(false, false) => "regular",
			(true, false) => "bold",
			(false, true) => "italic",
			(true, true) => "bold-italic",
		});
		path.set_extension("ttf");
		path
	}
}

impl TypeEditorMetadata for FontEditorMetadata {
	fn last_modified(&self, path: &Path) -> Result<SystemTime, AnyError> {
		let mut max_last_modified_at = path.metadata()?.modified()?;
		for path in [
			self.font_path(&path, false, false),
			self.font_path(&path, true, false),
			self.font_path(&path, false, true),
			self.font_path(&path, true, true),
		]
		.iter()
		.filter(|path| path.exists())
		{
			let last_modified_at = path.metadata()?.modified()?;
			max_last_modified_at = max_last_modified_at.max(last_modified_at);
		}
		Ok(max_last_modified_at)
	}

	fn read(&self, _path: &Path, json_str: &str) -> AssetResult {
		let font: Font = serde_json::from_str(json_str)?;
		Ok(Box::new(font))
	}

	fn compile(&self, json_path: &Path, asset: AnyBox) -> Result<Vec<u8>, AnyError> {
		use freetype::Library;
		let mut font = asset.downcast::<Font>().unwrap();

		// TODO: only initialize this once per build
		let font_library = Library::init()?;

		let sdf = SDFBuilder::default()
			.with_font_path(&self.font_path(json_path, false, false))
			.with_glyph_height(50)
			.with_spread(10)
			.with_padding(Vector::new([8; 4]))
			.with_minimum_atlas_size(Vector::new([1024, 512]))
			.build(&font_library)?;

		let mut png_path = json_path.parent().unwrap().to_path_buf();
		png_path.push(json_path.file_stem().unwrap());
		png_path.set_extension("png");

		let mut img = image::RgbaImage::new(sdf.size.x() as u32, sdf.size.y() as u32);
		for pos in sdf.size.iter(1) {
			img.put_pixel(
				pos.x() as u32,
				pos.y() as u32,
				image::Rgba([255, 255, 255, sdf.binary[pos.y()][pos.x()]]),
			);
		}
		img.save_with_format(png_path, image::ImageFormat::Png)?;

		font.set_sdf(sdf);

		let bytes = rmp_serde::to_vec(&font)?;
		Ok(bytes)
	}
}
