use curl::easy::Easy;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct ExternZipArtifact {
	glob: String,
	destination: String,
}

#[derive(Deserialize, Debug)]
struct ExternZip {
	url: String,
	contents: Vec<ExternZipArtifact>,
}

#[derive(Deserialize, Debug)]
struct ExternDownload {
	alias: String,
	downloads: Vec<ExternZip>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let ext_deps_file = fs::read_to_string("externs.json")?;
	let externs: Vec<ExternDownload> = serde_json::from_str(ext_deps_file.as_str())?;
	println!("{:?}", externs);

	for external in externs {
		for extern_zip in external.downloads {
			let mut download_dest = PathBuf::from("externs");
			download_dest.push(&external.alias);
			download_dest.push(PathBuf::from(&extern_zip.url).file_name().unwrap());

			println!("Removing any existing {:?}", download_dest);
			if let Err(err) = fs::remove_file(&download_dest) {
				match err.kind() {
					std::io::ErrorKind::NotFound => {}
					_ => panic!("{:?}", err),
				};
			}

			fetch_file(&extern_zip.url, download_dest)?;
		}
	}

	Ok(())
}

fn fetch_file(url: &String, destination: PathBuf) -> Result<(), Box<dyn Error>> {
	fs::create_dir_all(&destination.parent().unwrap())?;

	let mut file = match fs::File::open(&destination) {
		Ok(file) => file,
		Err(_) => fs::File::create(&destination)?,
	};

	println!("Downloading {:?}", url);
	let mut easy = Easy::new();
	easy.url(&url)?;
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
	Ok(())
}
