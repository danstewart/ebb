use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde::{Serialize, Deserialize};

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

pub fn exists() -> Result<bool> {
	// TODO
	Ok(false)
}

// Write the config HashMap as json to disk
pub fn write(config: &Config) -> Result<()> {
	let json = serde_json::to_string(&config)?;
	let mut file = File::create(".ebb-conf.json")?;
	file.write_all(json.as_bytes())?;
	Ok(())
}

// pub fn read() -> Result<HashMap<&str, String>> {

// }
