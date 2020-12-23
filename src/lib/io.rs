/// General IO
/// Creating templates, working out dirs, etc...
use std::path;
use anyhow::Result;
use std::io::prelude::*;

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

	let bytes = include_bytes!("../templates/wrapper.html");
	let mut wrapper_file = data_dir.clone();
	wrapper_file.push("wrapper.html");

	let mut file = std::fs::File::create(wrapper_file)?;
	file.write_all(bytes)?;

	Ok(())
}