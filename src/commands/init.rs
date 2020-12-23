use anyhow::Result;
use crate::lib::io;
use crate::lib::conf::{Config, Backend};
use crate::lib::prompt;
use crate::lib::prompt::PromptError::ValidateError;
use std::str::FromStr;

// Initial setup and configuration
pub fn init(args: &clap::ArgMatches) -> Result<()> {
	println!("ebb initialisation");
	println!("==================");

	make_config(args.is_present("force"))?;
	make_wrapper()?;

	// TODO: Open their editor to edit the wrapper
	// std::process::Command::new("code").spawn()?.wait()?;

	Ok(())
}

/// Ask user for input and store in config
fn make_config(force: bool) -> Result<()> {
	// If we have a valid config and haven't passed --force - bail
	if let Some(_) = Config::read() {
		if !force {
			println!("Config already exists, pass --force to overwrite existing config");
			return Ok(());
		}
	}

	// Ask user for the details we need
	let blog_name = prompt::ask("What is the name of your blog? ")?;

	let backend: Backend = prompt::validate("Which backend would you like to use? [s3/do] ", |ans: String| {
		match Backend::from_str(&ans.to_uppercase()[..]) {
			Ok(val) => Ok(val),
			Err(_)  => Err(ValidateError(String::from("Must be either S3 or DO")))
		}
	})?;

	// TODO: Check $EDITOR env var
	let editor = prompt::ask("What is the command for your preferred editor? ")?;

	let config = Config::new(blog_name, backend, editor);
	config.write()?;
	Ok(())
}

/// Copies the wrapper.html into the app data dir
fn make_wrapper() -> Result<()> {
	io::make_wrapper()?;
	Ok(())
}
