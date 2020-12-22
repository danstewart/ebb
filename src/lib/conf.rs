use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use dirs::config_dir;
use serde_json;
use serde::{Serialize, Deserialize};
use std::path;
use std::fs;

/// The supported storage backends
#[derive(Debug, Serialize, Deserialize)]
pub enum Backend {
	S3,
	DigitalOcean,
}

/// FromStr mapping for Backend enum
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

/// Config struct
// TODO: Should these be options or should they just have empty defaults??
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub blog_name: Option<String>,
	pub backend: Option<Backend>,
}

impl Default for Config {
	fn default() -> Config {
		Config { 
			blog_name: None,
			backend: None,
		}
	}
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

/// Return true if config file exists
fn exists() -> bool {
	return config_file().exists();
}

/// Instance of config
impl Config {
	/// Create new config instance
	pub fn new(blog_name: String, backend: Backend) -> Self {
		return Config {
			blog_name: Some(blog_name),
			backend: Some(backend),
		};
	}

	/// Return an empty config instance that can be populated later
	pub fn empty() -> Self {
		return Config::default();
	}

	/// Reads the config file and returns the Config struct
	/// Returns None if config is empty, invalid JSON or does not exist
	pub fn read() -> Option<Self> {
		let config_file = config_file();

		// If file doesn't exist then return None
		if !exists() {
			return None;
		}

		// If we have a config and we can read and parse it, return it
		if let Ok(contents) = std::fs::read_to_string(config_file) {
			if let Ok(json) = serde_json::from_str(&contents) {
				return Some(json);
			}
		}

		// Otherwise return none
		return None;
	}

	/// Write the config HashMap as json to disk
	pub fn write(self) -> Result<()> {
		let config_file = config_file();

		// We can safely unwrap this as config_file always returns a file
		fs::create_dir_all(config_file.parent().unwrap())?;
		let mut file = File::create(config_file)?;

		let json = serde_json::to_string(&self)?;
		file.write_all(json.as_bytes())?;
		Ok(())
	}
}
