#![warn(clippy::all)]

mod backends;
mod commands;
mod lib;

use anyhow::{anyhow, Result};
use clap::{App, Arg};
use commands::{add, init};
use tokio;

#[tokio::main]
async fn main() {
	let opts = App::new("ebb")
		.version("0.01")
		.author("Dan Stewart <danielandrewstewart@gmail.com>")
		.about("The easy blog builder")
		.subcommand(
			App::new("init").about("Initialises a new blog").arg(
				Arg::new("force")
					.long("force")
					.about("Forces overwriting existing config"),
			),
		)
		.subcommand(
			App::new("add").about("Creates a new blog post").arg(
				Arg::new("file")
					.index(1)
					.required(true)
					.about("The file name of the blog post"),
			),
		)
		.get_matches();

	if let Err(e) = dispatch(opts).await {
		eprintln!("{:?}\n\nPass --help for more info", e);
	}
}

// Dispatch our args to the appropriate action
async fn dispatch(opts: clap::ArgMatches) -> Result<()> {
	match opts.subcommand() {
		Some(("init", args)) => init::init(args).await,
		Some(("add", args)) => add::add(args),
		None => Err(anyhow!("A subcommand is required")),
		_ => Err(anyhow!("Invalid subcommand")),
	}
}
