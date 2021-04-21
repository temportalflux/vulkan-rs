use crate::asset;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use temportal_engine as engine;

/// Handles creating, saving, loading, moving, and deleting an asset at a given path.
/// Only accessible during editor-runtime whereas [Loader](temportal_engine::asset::Loader)
/// handles loading built assets during game-runtime.
pub struct Manager {
	editor_metadata: HashMap<engine::asset::TypeId, Box<dyn asset::TypeEditorMetadata>>,
}

impl Manager {
	pub fn new() -> Manager {
		Manager {
			editor_metadata: HashMap::new(),
		}
	}

	pub fn register<TAsset>(&mut self, editor_metadata: Box<dyn asset::TypeEditorMetadata>)
	where
		TAsset: engine::asset::Asset,
	{
		let runtime_metadata = TAsset::metadata();
		assert!(!self.editor_metadata.contains_key(runtime_metadata.name()));
		self.editor_metadata
			.insert(runtime_metadata.name(), editor_metadata);
	}

	/// Synchronously reads an asset json from a provided path, returning relevant asset loading errors.
	pub fn read_sync(
		&self,
		path: &Path,
	) -> Result<(String, engine::asset::AssetBox), engine::utility::AnyError> {
		let absolute_path = path.canonicalize()?;
		let file_json = fs::read_to_string(&absolute_path)?;
		let type_id = Manager::read_asset_type(file_json.as_str())?;
		let asset = self
			.editor_metadata
			.get(type_id.as_str())
			.ok_or(engine::asset::Error::UnregisteredAssetType(
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
		&self,
		json_path: &PathBuf,
		type_id: &String,
		asset: &engine::asset::AssetBox,
		write_to: &PathBuf,
	) -> Result<(), engine::utility::AnyError> {
		fs::create_dir_all(&write_to.parent().unwrap())?;
		let metadata = self.editor_metadata.get(type_id.as_str()).unwrap();
		let bytes = metadata.compile(&json_path, &asset)?;
		fs::write(write_to, bytes)?;
		Ok(())
	}
}
