use crate::lib::io::{data_dir, Dir};
use anyhow::Result;

// Add a new blog post file
pub fn add(args: &clap::ArgMatches) -> Result<()> {
	let mut _path = data_dir(Dir::Posts);
	println!("Adding file: {}", args.value_of("name").unwrap());
	Ok(())
}
