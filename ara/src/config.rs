/*-------------
/config.rs

This file is for the configuration of the application utilizing the clap crate.
-------------*/
use clap::Parser;

lazy_static! {
	pub static ref APP: Args = Args::parse();
}

#[derive(Parser, Debug, Clone)]
pub struct Args {
	/// Ignore Files Paths
	#[clap(long, value_delimiter = ' ', num_args = 1.., default_value = "/target /tests")]
	pub ignore: Vec<String>,

	/// Only runs the program on the specified function names Eg. "main foo bar"
	#[clap(long, value_delimiter = ' ', num_args = 1..)]
	pub function_name: Vec<String>,

	/// Debug mode
	#[clap(short, long, default_value = "false")]
	pub debug: bool,

	/// Verbose mode
	#[clap(short, default_value = "false")]
	pub verbose: bool,

	/// Path to the project
	#[clap(short, long, default_value = ".")]
	pub path: String,
}