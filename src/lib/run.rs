use anyhow::{Context, Result};
use std::path;

/// Collection of functions that start other applications
pub fn editor(editor: String, file: path::PathBuf) -> Result<()> {
	std::process::Command::new(&editor)
		.args(&[&file])
		.spawn()
		.with_context(|| format!("Failed to run '{} {}'", &editor, &file.display()))?
		.wait()?;

	Ok(())
}
