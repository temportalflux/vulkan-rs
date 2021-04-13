use curl::easy::Easy;
use extern_reader;
use regex::RegexSet;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use zip;

fn main() -> Result<(), Box<dyn Error>> {
	let ext_reader = extern_reader::ExternReader::new();
	let externs = ext_reader.get_externs()?;

	// Download all files in all externs
	for external in externs.iter() {
		for extern_zip in external.downloads.iter() {
			let download_dest = extern_zip.to_path(&ext_reader, &external);

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

			let downloaded_zip = extern_zip.to_path(&ext_reader, &external);
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

					let matched_globs = artifact_globs
						.matches(artifact_path.to_str().unwrap())
						.into_iter()
						.map(|matched_index| extern_zip.contents[matched_index].clone());
					for artifact_glob in matched_globs {
						let dest_file_path =
							artifact_glob.to_path(&ext_reader, &external, &artifact_path);
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
