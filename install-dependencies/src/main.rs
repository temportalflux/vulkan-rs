use curl::easy::Easy;
use serde_json;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
	let mut dependencies: Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();

	let ext_deps_file = fs::read_to_string("external-dependencies.json")?;
	if let serde_json::Value::Array(values) = serde_json::from_str(ext_deps_file.as_str())? {
		for value in values {
			if let serde_json::Value::Object(map) = value {
				dependencies.push(map);
			}
		}
	}

	let mut downloads: Vec<(String, PathBuf)> = Vec::new();
	for dependency in dependencies {
		if dependency.contains_key("download") {
			if let Some(serde_json::Value::String(download_url)) = dependency.get("download") {
				let url_path = PathBuf::from(download_url);
				let mut download_dest = PathBuf::from("dependencies");
				download_dest.push(dependency.get("alias").unwrap().as_str().unwrap());
				download_dest.push(url_path.file_name().unwrap());
				downloads.push((download_url.to_string(), download_dest));
			}
		}
	}

	for download in downloads {
		println!("Removing any existing {:?}", download.1);
		if let Err(err) = fs::remove_file(&download.1) {
			match err.kind() {
				std::io::ErrorKind::NotFound => {}
				_ => panic!("{:?}", err),
			};
		}

		let mut file = match fs::File::open(&download.1) {
			Ok(file) => file,
			Err(_) => fs::File::create(&download.1)?,
		};

		let mut easy = Easy::new();
		easy.url(&download.0)?;
		easy.write_function(move |data| {
			let mut pos_written = 0;
			if data.len() > 0 {
				pos_written += match file.write(&data) {
					Ok(bytes_written) => bytes_written,
					Err(_) => 0,
				};
			}
			Ok(pos_written)
		})?;
		easy.perform()?;
	}

	Ok(())
}
