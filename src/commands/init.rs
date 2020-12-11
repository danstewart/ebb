use anyhow::Result;

use crate::lib::conf;
use crate::lib::prompt;
use crate::lib::prompt::PromptError::ValidateError;
use std::str::FromStr;

// Initial setup and configuration
pub fn init() -> Result<()> {
	if conf::exists().unwrap() {
		return Ok(());
	}

	println!("ebb initialisation");
	println!("==================");

	let blog_name = prompt::ask("What is the name of your blog? ")?;
	let backend: conf::Backend = prompt::validate("Which backend would you like to use? [s3/do] ", |ans: String| {
		match conf::Backend::from_str(&ans.to_uppercase()[..]) {
			Ok(val) => Ok(val),
			Err(_)  => Err(ValidateError(String::from("Must be either S3 or DO")))
		}
	})?;

	let config = conf::Config {
		blog_name:  blog_name,
		backend: backend,
	};

	conf::write(&config)?;
	Ok(())
}
