mod lib;
mod commands;

use commands::{init, new};
use clap::{App, Arg};
use anyhow::{anyhow, Result};

fn main() {
	let opts = App::new("ebb")
					.version("0.01")
					.author("Dan Stewart <danielandrewstewart@gmail.com>")
					.about("The easy blog builder")
					.subcommand(App::new("init")
						.about("Initialises a new blog")
					)
					.subcommand(App::new("new")
						.about("Creates a new blog post")
						.arg(Arg::new("file")
							.index(1)
							.required(true)
							.about("The file name of the blog post")
						)
					)
					.get_matches();

	if let Err(e) = dispatch(opts) {
		eprintln!("{}\nPass --help for more info", e);
	}
}

// Dispatch our args to the appropriate action
fn dispatch(opts: clap::ArgMatches) -> Result<()> {
	match opts.subcommand() {
		Some(("init", _))   => init::init(),
		Some(("new", args)) => new::new(args),
		None                => Err(anyhow!("A subcommand is required")),
		_                   => Err(anyhow!("Invalid subcommand")),
	}
}
