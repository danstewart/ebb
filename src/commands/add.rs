use crate::lib::conf::Config;
use anyhow::Result;

pub fn add(args: &clap::ArgMatches) -> Result<()> {
	let config = Config::read();
	println!("{:?}", config);
	println!("Adding file: {}", args.value_of("file").unwrap());
	Ok(())
}
