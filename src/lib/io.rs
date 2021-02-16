use crate::lib::conf::Config;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::io::prelude::*;
/// General IO
/// Creating templates, working out dirs, etc...
use std::path;

// File names
const WRAPPER_FILE: &str = "wrapper.html";

// The different data dir sub directories
pub enum Dir {
	Root,  // The root of our data dir (eg. ~/.local/share/ebb/)
	Posts, // Where the raw (markdown) posts go (eg. /posts)
	Built, // Where the built (html) posts go (eg. /built)
}

impl std::string::ToString for Dir {
	fn to_string(&self) -> String {
		match self {
			Self::Root => String::from(""),
			Self::Posts => String::from("posts"),
			Self::Built => String::from("built"),
		}
	}
}

/// Returns the PathBuf for the specified data dir
/// Panics if unable to detect data dir
/// https://docs.rs/dirs/3.0.1/dirs/fn.data_dir.html
pub fn data_dir(dir_type: Dir) -> path::PathBuf {
	if let Some(mut data_dir) = dirs::data_dir() {
		data_dir.push("ebb");
		data_dir.push(dir_type.to_string());

		return data_dir;
	}

	panic!("Unable to determine app data directory");
}

/// Returns the config file path
/// Panics if unable to detect config home dir
/// https://docs.rs/dirs/3.0.1/dirs/fn.config_dir.html
pub fn config_file() -> path::PathBuf {
	if let Some(mut config_home) = dirs::config_dir() {
		config_home.push("ebb");
		config_home.push("config.json");
		return config_home;
	}

	panic!("Unable to determine config file path");
}

/// Writes the default wrapper.html file to the app data dir
pub fn make_wrapper() -> Result<()> {
	// Load the config
	let config = Config::read().ok_or(anyhow!("No config found when building wrapper file"))?;

	// Values to replace into the wrapper.html
	let mut replacers = HashMap::new();
	replacers.insert("author", &config.author);
	replacers.insert("title", &config.blog_name);

	// Read our template
	let bytes = include_bytes!("../templates/wrapper.html"); // Annoyingly this must be a raw str
	let mut wrapper = String::from_utf8(bytes.to_vec())?;

	// Replace our vars
	for (key, val) in &replacers {
		let mut tag = String::from("{{ ");
		tag.push_str(key);
		tag.push_str(" }}");
		wrapper = wrapper.replace(&tag, val);
	}

	// Write formatted template to disk
	let mut wrapper_file = data_dir(Dir::Root);
	wrapper_file.push(WRAPPER_FILE);
	write_file(wrapper_file, wrapper)?;

	Ok(())
}

/// Write string to a file
pub fn write_file(path: path::PathBuf, contents: String) -> Result<()> {
	let mut file = touch(&path)?;
	file.write_all(contents.as_bytes())
		.with_context(|| format!("Failed to write to {}", path.display()))?;

	Ok(())
}

/// Create an empty file and it's directory structure
pub fn touch(path: &path::PathBuf) -> Result<std::fs::File> {
	if let Some(parent_dir) = &path.parent() {
		std::fs::create_dir_all(parent_dir)?;
	}

	Ok(std::fs::File::create(path)?)
}

/// Return path of wrapper file
pub fn wrapper_file() -> path::PathBuf {
	let mut path = data_dir(Dir::Root);
	path.push(WRAPPER_FILE);
	path
}
