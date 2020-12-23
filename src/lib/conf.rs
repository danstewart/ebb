use anyhow::{Result, Context};
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::lib::io;

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
	pub blog_name: String,
	pub backend: Backend,
	pub editor: String,
}

/// Return true if config file exists
fn exists() -> bool {
	return io::config_file().exists();
}

/// Instance of config
impl Config {
	/// Create new config instance
	pub fn new(blog_name: String, backend: Backend, editor: String) -> Self {
		return Config {
			blog_name: blog_name,
			backend: backend,
			editor: editor,
		};
	}

	/// Reads the config file and returns the Config struct
	/// Returns None if config is empty, invalid JSON or does not exist
	pub fn read() -> Option<Self> {
		let config_file = io::config_file();

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
	pub fn write(&self) -> Result<()> {
		let config_file = io::config_file();

		// We can safely unwrap this as config_file always returns a file
		fs::create_dir_all(config_file.parent().unwrap())?;
		let mut file = File::create(&config_file)?;

		let json = serde_json::to_string(&self)
			.with_context(|| "Failed to serialize config struct")?;

		file.write_all(json.as_bytes())
			.with_context(|| format!("Failed to write config file to {}", config_file.display()))?;
		Ok(())
	}
}
