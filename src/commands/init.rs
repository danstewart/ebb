use anyhow::Result;

use crate::lib::conf::{Config, Backend};
use crate::lib::prompt;
use crate::lib::prompt::PromptError::ValidateError;
use std::str::FromStr;

// Initial setup and configuration
pub fn init(args: &clap::ArgMatches) -> Result<()> {
	if let Some(_) = Config::read() {
		if args.occurrences_of("force") == 0 {
			println!("Config already exists, pass --force to overwrite existing config");
			return Ok(());
		}
	}

	println!("ebb initialisation");
	println!("==================");

	let blog_name = prompt::ask("What is the name of your blog? ")?;
	let backend: Backend = prompt::validate("Which backend would you like to use? [s3/do] ", |ans: String| {
		match Backend::from_str(&ans.to_uppercase()[..]) {
			Ok(val) => Ok(val),
			Err(_)  => Err(ValidateError(String::from("Must be either S3 or DO")))
		}
	})?;

	let config = Config::new(blog_name, backend);
	config.write()?;
	Ok(())
}
