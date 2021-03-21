pub mod markdown;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// List of valid format types
// Each of these MUST have a mapping in FormatType::from_str
pub const VALID_FORMATS: &'static [&'static str; 2] = &["md", "html"];

/// The supported storage backends
#[derive(Debug, Serialize, Deserialize)]
pub enum FormatType {
	MD,
	HTML,
}

/// FromStr mapping for FormatType enum
impl std::str::FromStr for FormatType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"MD" => Ok(FormatType::MD),
			"HTML" => Ok(FormatType::HTML),
			_ => Err(format!("'{}' is not a valid format", s)),
		}
	}
}

// The Format trait all formats must impl
#[async_trait]
pub trait Format {
	fn new() -> Self;

	/// Convert to HTML and surround with wrapper.html
	async fn build(&self) -> Result<()>;
}

