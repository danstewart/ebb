use crate::lib::conf::Config;
use crate::lib::io::{data_dir, Dir};
use crate::lib::run;
use anyhow::{anyhow, Result};

// Add a new blog post file
pub fn add(args: &clap::ArgMatches) -> Result<()> {
	let name = match args.value_of("name") {
		Some(name) => name,
		None => return Err(anyhow!("Name is required")),
	};

	// TODO: More validation
	if name.contains("/") {
		return Err(anyhow!("Post name must not contain '/'"));
	}

	let mut path = data_dir(Dir::Posts);
	std::fs::create_dir_all(&path)?;
	path.push(name);

	let editor = match Config::read() {
		Some(config) => config.editor,
		None => String::from("vi"),
	};

	run::editor(editor, path)?;

	Ok(())
}
