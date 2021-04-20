use std::{
	self, fs,
	io::{self},
	path::{Path, PathBuf},
};
use temportal_engine as engine;

pub fn build(
	asset_manager: &crate::asset::Manager,
	module_name: &str,
) -> Result<(), engine::utility::AnyError> {
	let crate_path = [std::env!("CARGO_MANIFEST_DIR"), "..", module_name]
		.iter()
		.collect::<PathBuf>()
		.canonicalize()?;
	let mut assets_dir_path = crate_path.clone();
	assets_dir_path.push("assets");
	let mut output_dir_path = crate_path.clone();
	output_dir_path.push("binaries");

	fs::create_dir(&output_dir_path)?;
	fs::remove_dir_all(&output_dir_path)?;

	for file_path in collect_file_paths(&assets_dir_path)?.iter() {
		let relative_path = file_path.as_path().strip_prefix(&assets_dir_path)?;
		if let Some(ext) = relative_path.extension() {
			if ext == "json" {
				let mut output_file_path = [output_dir_path.as_path(), relative_path]
					.iter()
					.collect::<PathBuf>();
				output_file_path.set_extension("bin");
				let (type_id, asset) = asset_manager.read_sync(&file_path.as_path())?;
				asset_manager.compile(&file_path, &type_id, &asset, &output_file_path)?;
			}
		}
	}
	Ok(())
}

pub fn collect_file_paths(path: &Path) -> io::Result<Vec<PathBuf>> {
	let mut file_paths: Vec<PathBuf> = Vec::new();
	if !path.is_dir() {
		return Ok(file_paths);
	}

	let mut directory_paths: Vec<PathBuf> = vec![path.to_path_buf()];
	while directory_paths.len() > 0 {
		for entry in fs::read_dir(directory_paths.pop().unwrap())? {
			let entry_path = entry?.path();
			if entry_path.is_dir() {
				directory_paths.push(entry_path.to_path_buf());
			} else {
				file_paths.push(entry_path.to_path_buf());
			}
		}
	}

	Ok(file_paths)
}

pub fn get_output_dir(module: &str) -> Result<std::path::PathBuf, engine::utility::AnyError> {
	let mut workspace_path = std::env::current_dir()?;
	workspace_path.push(module);
	workspace_path.push("src");
	Ok(workspace_path)
}
