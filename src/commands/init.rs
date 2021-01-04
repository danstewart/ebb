use crate::lib::conf::{Backend, Config};
use crate::lib::io;
use crate::lib::prompt;
use crate::lib::prompt::PromptError::ValidateError;
use anyhow::{anyhow, Context, Result};
use std::str::FromStr;

// Initial setup and configuration
pub fn init(args: &clap::ArgMatches) -> Result<()> {
	println!("ebb initialisation");
	println!("==================");

	let config = make_config(args.is_present("force"))?;
	make_wrapper()?;

	// Open their selected editor to edit the wrapper
	std::process::Command::new(&config.editor)
		.args(&[io::wrapper_file()])
		.spawn()
		.with_context(|| {
			format!(
				"Failed to run '{} {}'",
				&config.editor,
				io::wrapper_file().display()
			)
		})?
		.wait()?;

	Ok(())
}

/// Ask user for input and store in config
/// Returns Err() if config exists
fn make_config(force: bool) -> Result<Config> {
	// If we have a valid config and haven't passed --force then bail
	if Config::read().is_some() && !force {
		let error = anyhow!("Config already exists, use `ebb init --force` to overwrite existing config or use `ebb edit wrapper` to edit the wrapper.html");
		return Err(error);
	}

	// Ask user for the details we need
	let blog_name = prompt::ask("What is the name of your blog? ")?;

	let backend: Backend = prompt::validate(
		"Which backend would you like to use? [s3/do] ",
		|ans: String| match Backend::from_str(&ans.to_uppercase()[..]) {
			Ok(val) => Ok(val),
			Err(_) => Err(ValidateError(String::from("Must be either S3 or DO"))),
		},
	)?;

	// TODO: Check $EDITOR env var
	let editor = prompt::ask("What is the command for your preferred editor? ")?;

	let config = Config::new(blog_name, backend, editor);
	config.write()?;
	Ok(config)
}

/// Copies the wrapper.html into the app data dir
fn make_wrapper() -> Result<()> {
	io::make_wrapper()?;
	Ok(())
}
