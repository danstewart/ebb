use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use dirs::config_dir;
use serde_json;
use serde::{Serialize, Deserialize};
use std::path;
use std::fs;

#[derive(Serialize, Deserialize)]
pub enum Backend {
	S3,
	DigitalOcean,
}

impl std::str::FromStr for Backend {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"S3" => Ok(Backend::S3),
				"DO" => Ok(Backend::DigitalOcean),
				_ => Err(format!("'{}' is not a valid backend", s))
			}
	}
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub blog_name: String,
	pub backend: Backend
}

pub fn exists() -> bool {
	return config_file().exists();
}

/// Write the config HashMap as json to disk
pub fn write(config: &Config) -> Result<()> {
	let config_file = config_file();

	// We can safely unwrap this as config_file always returns a file
	fs::create_dir_all(config_file.parent().unwrap())?;
	let mut file = File::create(config_file)?;

	let json = serde_json::to_string(&config)?;
	file.write_all(json.as_bytes())?;
	Ok(())
}

/// Returns the config file path
/// Panics if unable to detect config home dir
fn config_file() -> path::PathBuf {
	if let Some(mut config_home) = config_dir() {
		config_home.push("ebb");
		config_home.push("config.json");
		return config_home;
	}

	panic!("Unable to determine config file path");
}


// pub fn read() -> Result<HashMap<&str, String>> {

// }
