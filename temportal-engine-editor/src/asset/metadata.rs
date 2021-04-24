use crate::engine::{asset, utility::AnyError};

pub trait TypeEditorMetadata {
	fn read(&self, path: &std::path::Path, json_str: &str) -> asset::AssetResult;
	fn compile(
		&self,
		json_path: &std::path::Path,
		asset: &asset::AssetBox,
	) -> Result<Vec<u8>, AnyError>;
}
