pub mod s3;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// List of valid backend types
// Each of these MUST have a mapping in BackendType::from_str
pub const VALID_BACKENDS: &'static [&'static str; 2] = &["s3", "do"];

/// The supported storage backends
#[derive(Debug, Serialize, Deserialize)]
pub enum BackendType {
	S3,
	DigitalOcean,
}

/// FromStr mapping for Backend enum
impl std::str::FromStr for BackendType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"S3" => Ok(BackendType::S3),
			"DO" => Ok(BackendType::DigitalOcean),
			_ => Err(format!("'{}' is not a valid backend", s)),
		}
	}
}

// The Backend trait all backends must impl
#[async_trait]
pub trait Backend {
	fn new() -> Self;

	/// Initialise new backend
	/// Gather keys etc...
	async fn init(&self) -> Result<()>;

	/// Publish blog posts
	fn publish(&self);

	/// Create a local zip of all blog posts
	fn backup(&self);
}
