use crate::asset;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use temportal_engine as engine;

/// Handles creating, saving, loading, moving, and deleting an asset at a given path.
/// Only accessible during editor-runtime whereas [Loader](temportal_engine::asset::Loader)
/// handles loading built assets during game-runtime.
pub struct Manager {}

impl Manager {
	/// Synchronously reads an asset json from a provided path, returning relevant asset loading errors.
	pub fn read_sync(
		registry: &engine::asset::TypeRegistry,
		path: &Path,
	) -> Result<(String, engine::asset::AssetBox), engine::utility::AnyError> {
		let absolute_path = path.canonicalize()?;
		let file_json = fs::read_to_string(&absolute_path)?;
		let type_id = Manager::read_asset_type(file_json.as_str())?;
		let asset = registry
			.get(type_id.as_str())
			.ok_or(engine::asset::Error::UnregisteredAssetType(
				absolute_path.clone(),
				type_id.to_string(),
			))?
			.read(&absolute_path, file_json.as_str())?;
		Ok((type_id, asset))
	}

	fn read_asset_type(json_str: &str) -> Result<String, engine::utility::AnyError> {
		let generic: engine::asset::AssetGeneric = serde_json::from_str(json_str)?;
		return Ok(generic.asset_type);
	}

	pub fn compile(
		registry: &engine::asset::TypeRegistry,
		json_path: &PathBuf,
		type_id: &String,
		asset: &engine::asset::AssetBox,
		write_to: &PathBuf,
	) -> Result<(), engine::utility::AnyError> {
		fs::create_dir_all(&write_to.parent().unwrap())?;
		let metadata = registry.get(type_id).unwrap();
		let bytes = metadata.compile(&json_path, &asset)?;
		fs::write(write_to, bytes)?;
		Ok(())
	}
}
