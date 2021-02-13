use crate::lib::conf::Config;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::io::prelude::*;
/// General IO
/// Creating templates, working out dirs, etc...
use std::path;

// File names
const WRAPPER_FILE: &str = "wrapper.html";

/// Returns the app data dir
/// Panics if unable to detect data dir
/// https://docs.rs/dirs/3.0.1/dirs/fn.data_dir.html
pub fn data_dir() -> path::PathBuf {
	if let Some(mut data_dir) = dirs::data_dir() {
		data_dir.push("ebb");
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
	let data_dir = data_dir();
	std::fs::create_dir_all(&data_dir)?;

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
	let mut wrapper_file = data_dir;
	wrapper_file.push(WRAPPER_FILE);

	let mut file = std::fs::File::create(&wrapper_file)?;
	file.write_all(wrapper.as_bytes())
		.with_context(|| format!("Failed to write wrapper.html to {}", wrapper_file.display()))?;

	Ok(())
}

/// Return path of wrapper file
pub fn wrapper_file() -> path::PathBuf {
	let mut path = data_dir();
	path.push(WRAPPER_FILE);
	path
}
