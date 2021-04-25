use crate::{
	asset::TypeEditorMetadata,
	engine::{
		asset::{as_asset, AssetBox, AssetResult},
		graphics::Font,
		math::Vector,
		utility::AnyError,
	},
	graphics::FontSDFBuilder,
};
use serde_json;
use std::path::{Path, PathBuf};

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
	fn read(&self, _path: &Path, json_str: &str) -> AssetResult {
		let font: Font = serde_json::from_str(json_str)?;
		Ok(Box::new(font))
	}

	fn compile(&self, json_path: &Path, asset: &AssetBox) -> Result<Vec<u8>, AnyError> {
		use freetype::Library;
		let asset = as_asset::<Font>(asset).clone();

		// TODO: only initialize this once per build
		let font_library = Library::init()?;

		let (atlas_size, atlas_binary) = FontSDFBuilder::default()
			.with_font_path(&self.font_path(json_path, false, false))
			.with_glyph_height(60)
			.with_spread(10)
			.with_padding(Vector::new([8; 4]))
			.with_minimum_atlas_size(Vector::new([1024, 512]))
			.build(&font_library)?;

		// TODO: Temporary, dont need to create an image when the binary can be embedded in the asset bin
		let mut img = image::RgbaImage::new(atlas_size.x() as u32, atlas_size.y() as u32);
		for x in 0..atlas_size.x() {
			for y in 0..atlas_size.y() {
				img.put_pixel(
					x as u32,
					y as u32,
					image::Rgba([255, 255, 255, atlas_binary[y][x]]),
				);
			}
		}
		img.save_with_format(PathBuf::from("font.png"), image::ImageFormat::Png)?;

		let bytes = rmp_serde::to_vec(&asset)?;
		Ok(bytes)
	}
}
