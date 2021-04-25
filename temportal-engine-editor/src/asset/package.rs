use crate::engine::utility::VoidResult;
use std::{self, fs, io::Write, path::PathBuf};
use zip;

pub fn package(module_name: &str) -> VoidResult {
	let module_dir = [std::env!("CARGO_MANIFEST_DIR"), "..", module_name]
		.iter()
		.collect::<PathBuf>()
		.canonicalize()?;
	let mut output_dir_path = module_dir.clone();
	output_dir_path.push("binaries");
	let mut zip_path = module_dir.clone();
	zip_path.push(format!("{}.pak", module_name));

	let zip_file = fs::OpenOptions::new()
		.write(true)
		.create(true)
		.open(&zip_path)?;
	let mut zipper = zip::ZipWriter::new(zip_file);
	let zip_options =
		zip::write::FileOptions::default().compression_method(zip::CompressionMethod::BZIP2);

	for file_path in crate::asset::build::collect_file_paths(&output_dir_path, &Vec::new())?.iter() {
		let relative_path = file_path
			.as_path()
			.strip_prefix(&output_dir_path)?
			.to_str()
			.unwrap();
		let bytes = fs::read(&file_path)?;
		zipper.start_file(relative_path, zip_options)?;
		zipper.write_all(&bytes[..])?;
	}

	zipper.finish()?;

	Ok(())
}
