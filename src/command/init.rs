use crate::backend::{s3::S3, Backend, BackendType, VALID_BACKENDS};
use crate::lib::conf::Config;
use crate::lib::io;
use crate::lib::run;
use anyhow::{anyhow, Result};
use clinput::Prompt;
use std::str::FromStr;

// Initial setup and configuration
pub async fn init(args: &clap::ArgMatches) -> Result<()> {
	println!("ebb initialisation");
	println!("==================");

	let config = make_config(args.is_present("force"))?;
	make_wrapper()?;

	// Open their selected editor to edit the wrapper
	run::editor(config.editor, io::wrapper_file())?;

	let s3 = S3::new();
	println!("Backend init ok? {}", s3.init().await.is_ok());
	Ok(())
}

/// Ask user for input and store in config
/// Returns Err() if config exists
fn make_config(force: bool) -> Result<Config> {
	// If we have a valid config and haven't passed --force then bail
	if !force && Config::read().is_some() {
		let error = anyhow!("Config already exists, use `ebb init --force` to overwrite existing config or use `ebb edit wrapper` to edit the wrapper.html");
		return Err(error);
	}

	// Ask user for the details we need
	let author = Prompt::not_blank().ask("What is your name? ")?;
	let blog_name = Prompt::not_blank().ask("What is the name of your blog? ")?;

	let backend = Prompt::new()
		.choices(VALID_BACKENDS.to_vec())
		.ask("Which backend would you like to use? ")?;
	let backend = BackendType::from_str(backend.as_str()).unwrap();

	// TODO: Check $EDITOR env var
	let editor = Prompt::not_blank().ask("What is the command for your preferred editor? ")?;

	let config = Config::new(author, blog_name, backend, editor);
	config.write()?;
	Ok(config)
}

/// Copies the wrapper.html into the app data dir
fn make_wrapper() -> Result<()> {
	io::make_wrapper()?;
	Ok(())
}
