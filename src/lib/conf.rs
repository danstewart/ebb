use crate::lib::io;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use crate::backend::BackendType;
use crate::format::FormatType;

// NOTE: This used to be a singleton using the OnceCell lib
// It turned out to be overly clunky but noting here in case
// it's needed at some point.
// New strategy is to just re-read the config each time.

/// Config struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub author: String,
	pub blog_name: String,
	pub backend: BackendType,
	pub format: FormatType,
	pub editor: String,
}

/// Return true if config file exists
fn exists() -> bool {
	io::config_file().exists()
}

/// Instance of config
impl Config {
	/// Create new config instance
	pub fn new(author: String, blog_name: String, backend: BackendType, editor: String) -> Self {
		Config {
			author,
			blog_name,
			backend,
			format: FormatType::MD,
			editor,
		}
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
		None
	}

	/// Write the config HashMap as json to disk
	pub fn write(&self) -> Result<()> {
		let config_file = io::config_file();

		// We can safely unwrap this as config_file always returns a file
		fs::create_dir_all(config_file.parent().unwrap())?;
		let mut file = File::create(&config_file)?;

		let json =
			serde_json::to_string(&self).with_context(|| "Failed to serialize config struct")?;

		file.write_all(json.as_bytes())
			.with_context(|| format!("Failed to write config file to {}", config_file.display()))?;
		Ok(())
	}
}
