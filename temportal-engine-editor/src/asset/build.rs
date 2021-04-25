use std::{
	self, fs,
	io::{self},
	path::{Path, PathBuf},
};
use crate::engine::{self, asset};

pub fn build(
	asset_manager: &crate::asset::Manager,
	module_name: &str,
) -> engine::utility::VoidResult {
	log::info!(target: asset::LOG, "Building module {}", module_name);
	let crate_path = [std::env!("CARGO_MANIFEST_DIR"), "..", module_name]
		.iter()
		.collect::<PathBuf>()
		.canonicalize()?;
	let mut assets_dir_path = crate_path.clone();
	assets_dir_path.push("assets");
	let mut output_dir_path = crate_path.clone();
	output_dir_path.push("binaries");

	if !output_dir_path.exists() {
		fs::create_dir(&output_dir_path)?;
	}
	
	let mut intended_binaries: Vec<PathBuf> = Vec::new();
	for asset_file_path in collect_file_paths(&assets_dir_path, &Vec::new())?.iter() {
		let relative_path = asset_file_path.as_path().strip_prefix(&assets_dir_path)?;
		if let Some(ext) = relative_path.extension() {
			if ext == "json" {
				let mut binary_file_path = output_dir_path.clone();
				if let Some(parent) = relative_path.parent() {
					binary_file_path.push(parent);
				}
				binary_file_path.push(relative_path.file_stem().unwrap());

				if !binary_file_path.exists()
					|| (asset_manager.last_modified(&asset_file_path)?
						> binary_file_path.metadata()?.modified()?)
				{
					log::info!(target: asset::LOG, "Building asset {:?}", relative_path);
					let (type_id, asset) = asset_manager.read_sync(&asset_file_path.as_path())?;
					asset_manager.compile(&asset_file_path, &type_id, &asset, &binary_file_path)?;
				} else {
					log::info!(target: asset::LOG, "Skipping unchanged asset {:?}", relative_path);
				}

				intended_binaries.push(binary_file_path);
			}
		}
	}

	for binary_file_path in collect_file_paths(&output_dir_path, &intended_binaries)?.iter() {
		log::info!(target: asset::LOG, "Deleting old binary {:?}", binary_file_path.as_path().strip_prefix(&output_dir_path)?);
		std::fs::remove_file(binary_file_path)?;
	}

	Ok(())
}

pub fn collect_file_paths(path: &Path, ignore: &Vec<PathBuf>) -> io::Result<Vec<PathBuf>> {
	let mut file_paths: Vec<PathBuf> = Vec::new();
	if !path.is_dir() {
		return Ok(file_paths);
	}

	let mut directory_paths: Vec<PathBuf> = vec![path.to_path_buf()];
	while directory_paths.len() > 0 {
		for entry in fs::read_dir(directory_paths.pop().unwrap())? {
			let entry_path = entry?.path().to_path_buf();
			if entry_path.is_dir() {
				directory_paths.push(entry_path);
			} else if !ignore.contains(&entry_path) {
				file_paths.push(entry_path);
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
