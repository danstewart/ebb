use anyhow::Result;

pub fn add(args: &clap::ArgMatches) -> Result<()> {
	println!("Adding file: {}", args.value_of("file").unwrap());
	Ok(())
}
