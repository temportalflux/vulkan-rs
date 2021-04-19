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
	) -> engine::asset::AssetResult {
		let absolute_path = path.canonicalize()?;
		let file_json = fs::read_to_string(&absolute_path)?;
		let type_id = Manager::read_asset_type(&absolute_path, file_json.as_str())?;
		registry
			.get(type_id.as_str())
			.ok_or(asset::Error::UnregisteredAssetType(
				absolute_path.clone(),
				type_id,
			))?
			.read(&absolute_path, file_json.as_str())
	}

	fn read_asset_type(
		path: &PathBuf,
		json_str: &str,
	) -> Result<String, engine::utility::AnyError> {
		let parsed_json: serde_json::Value = serde_json::from_str(json_str)?;
		match parsed_json.as_object() {
			Some(obj) => match obj.get("type") {
				Some(id) => Ok(id.as_str().unwrap().to_string()),
				None => return Err(Box::new(asset::Error::MissingTypeId(path.clone()))),
			},
			None => {
				return Err(Box::new(asset::Error::InvalidJson(
					path.clone(),
					"not a json object".to_string(),
				)))
			}
		}
	}
}
