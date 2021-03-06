use crate::lib::conf::Config;
use crate::lib::io::{data_dir, Dir};
use crate::lib::run;
use anyhow::{anyhow, Result};
use clinput::Prompt;
use regex::Regex;

// Add a new blog post file
pub fn add(args: &clap::ArgMatches) -> Result<()> {
	let name: String = match args.value_of("name") {
		Some(name) => String::from(name),
		None => Prompt::not_blank().ask("What is the name of the post?")?,
	};

	// Validate name format
	let rule = r"^[\w\d\-_\.]+$";
	let re = Regex::new(rule)?;
	if !re.is_match(&name) {
		return Err(anyhow!(
			"Post name must only contain letters, numbers, '-', '_' and '.'"
		));
	}

	let mut path = data_dir(Dir::Posts);
	std::fs::create_dir_all(&path)?;
	path.push(name);

	// TODO: Check if file already exists

	let editor = match Config::read() {
		Some(config) => config.editor,
		None => String::from("vi"),
	};

	run::editor(editor, path)?;
	Ok(())
}
