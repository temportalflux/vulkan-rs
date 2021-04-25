use crate::{
	asset::TypeEditorMetadata,
	engine::{
		asset::{as_asset, AssetBox, AssetResult},
		graphics::font::Font,
		math::Vector,
		utility::AnyError,
	},
};
use serde_json;
use std::path::{Path, PathBuf};

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
	fn read(&self, _path: &Path, json_str: &str) -> AssetResult {
		let font: Font = serde_json::from_str(json_str)?;
		Ok(Box::new(font))
	}

	fn compile(&self, json_path: &Path, asset: &AssetBox) -> Result<Vec<u8>, AnyError> {
		use freetype::Library;
		let mut asset = as_asset::<Font>(asset).clone();

		// TODO: only initialize this once per build
		let font_library = Library::init()?;

		asset.set_sdf(
			SDFBuilder::default()
				.with_font_path(&self.font_path(json_path, false, false))
				.with_glyph_height(60)
				.with_spread(10)
				.with_padding(Vector::new([8; 4]))
				.with_minimum_atlas_size(Vector::new([1024, 512]))
				.build(&font_library)?,
		);

		println!("{:?}", asset);

		let bytes = rmp_serde::to_vec(&asset)?;
		Ok(bytes)
	}
}
