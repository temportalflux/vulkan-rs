use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Extern {
	pub alias: String,
	pub url: String,
	pub downloads: Vec<Artifact>,
}

#[derive(Deserialize, Debug)]
pub struct Artifact {
	pub url: String,
	pub contents: Vec<ArtifactGlob>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ArtifactGlob {
	pub glob: String,
	pub destination: String,
}

#[derive(Debug)]
pub struct ExternReader {
	path_to_root: PathBuf,
}

impl ExternReader {
	pub fn new() -> ExternReader {
		ExternReader {
			path_to_root: PathBuf::new(),
		}
	}

	pub fn set_root(&mut self, path_to_root: Option<String>) {
		self.path_to_root = match path_to_root {
			Some(path) => PathBuf::from(path),
			None => PathBuf::new(),
		};
	}

	pub fn get_externs(&self) -> Result<Vec<Extern>, Box<dyn Error>> {
		let mut extern_path = self.path_to_root.clone();
		extern_path.push("externs.json");
		let ext_deps_file = fs::read_to_string(extern_path)?;
		let externs: Vec<Extern> = serde_json::from_str(ext_deps_file.as_str())?;
		Ok(externs)
	}

	pub fn extern_dir(&self) -> PathBuf {
		let mut extern_path = self.path_to_root.clone();
		extern_path.push("externs");
		extern_path
	}
}

impl Artifact {
	pub fn to_path(&self, reader: &ExternReader, external: &Extern) -> PathBuf {
		let mut path = reader.extern_dir();
		path.push(&external.alias);
		path.push(PathBuf::from(&self.url).file_name().unwrap());
		path
	}
}

impl ArtifactGlob {
	pub fn to_path(
		&self,
		reader: &ExternReader,
		external: &Extern,
		path_in_archive: &PathBuf,
	) -> PathBuf {
		let mut path = reader.extern_dir();
		path.push(&external.alias);
		path.push(&self.destination);
		path.push(path_in_archive.file_name().unwrap());
		path
	}
}
