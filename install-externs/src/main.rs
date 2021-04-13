use curl::easy::Easy;
use regex::RegexSet;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use zip;

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

#[derive(Debug)]
struct MatchedArtifact {
	path_in_zip: PathBuf,
	destination: String,
}

fn make_destination_path(external: &ExternDownload, extern_zip: &ExternZip) -> PathBuf {
	let mut path = PathBuf::from("externs");
	path.push(&external.alias);
	path.push(PathBuf::from(&extern_zip.url).file_name().unwrap());
	path
}

fn make_artifact_path(
	external: &ExternDownload,
	extern_dest: &String,
	path_in_zip: &PathBuf,
) -> PathBuf {
	let mut path = PathBuf::from("externs");
	path.push(&external.alias);
	path.push(extern_dest);
	path.push(path_in_zip.file_name().unwrap());
	path
}

fn main() -> Result<(), Box<dyn Error>> {
	let ext_deps_file = fs::read_to_string("externs.json")?;
	let externs: Vec<ExternDownload> = serde_json::from_str(ext_deps_file.as_str())?;

	// Download all files in all externs
	for external in externs.iter() {
		for extern_zip in external.downloads.iter() {
			let download_dest = make_destination_path(&external, &extern_zip);

			println!("Removing any existing {:?}", download_dest);
			if let Err(err) = fs::remove_file(&download_dest) {
				match err.kind() {
					std::io::ErrorKind::NotFound => {}
					_ => panic!("{:?}", err),
				};
			}

			fetch_file(&extern_zip.url, &download_dest)?;
		}
	}

	// Parse the contents for all downloaded externs
	for external in externs.iter() {
		for extern_zip in external.downloads.iter() {
			let artifact_globs = extern_zip.contents.iter().map(|item| item.glob.as_str());
			let artifact_globs = RegexSet::new(artifact_globs).unwrap();

			let downloaded_zip = make_destination_path(&external, &extern_zip);
			println!("Processing contents of {:?}", downloaded_zip);

			{
				let file = fs::File::open(&downloaded_zip)?;
				let mut archive = zip::ZipArchive::new(file).unwrap();
				for i in 0..archive.len() {
					let mut artifact_file = archive.by_index(i).unwrap();
					let artifact_path = match artifact_file.enclosed_name() {
						Some(path) => path.to_owned(),
						None => continue,
					};

					let matched_dest_paths = artifact_globs
						.matches(artifact_path.to_str().unwrap())
						.into_iter()
						.map(|matched_index| {
							extern_zip.contents[matched_index].destination.clone()
						});
					for destination_path in matched_dest_paths {
						let dest_file_path =
							make_artifact_path(&external, &destination_path, &artifact_path);
						println!("  Extracting {:?} to {:?}", artifact_path, dest_file_path);

						fs::create_dir_all(&dest_file_path.parent().unwrap())?;

						if let Err(err) = fs::remove_file(&dest_file_path) {
							match err.kind() {
								std::io::ErrorKind::NotFound => {}
								_ => panic!("{:?}", err),
							};
						}

						let mut file_at_destination = match fs::File::open(&dest_file_path) {
							Ok(file) => file,
							Err(_) => fs::File::create(&dest_file_path)?,
						};

						std::io::copy(&mut artifact_file, &mut file_at_destination).unwrap();
					}
				}
			}

			println!(
				"  Finished processing \"{:?}\". It will now be deleted.",
				downloaded_zip
			);
			if let Err(err) = fs::remove_file(&downloaded_zip) {
				match err.kind() {
					std::io::ErrorKind::NotFound => {}
					_ => panic!("{:?}", err),
				};
			}
		}
	}

	Ok(())
}

fn fetch_file(url: &String, destination: &PathBuf) -> Result<(), Box<dyn Error>> {
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
