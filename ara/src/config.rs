/*-------------
/config.rs

This file is for the configuration of the application utilizing the clap crate.
-------------*/
use std::cell::OnceCell;

pub const APP: OnceCell<Settings> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct Settings {
	/// Ignore Files Paths
	pub ignore: Vec<String>,

	/// Only runs the program on the specified function names Eg. "main foo bar"
	pub function_name: Vec<String>,

	/// Debug mode
	pub debug: bool,

	/// Verbose mode
	pub verbose: bool,

	/// Path to the project
	pub path: String,
}

impl Settings {
	/// Create a new Settings struct
	pub fn new() -> Settings {
		Settings {
			ignore: vec!["/target".to_string(), "/tests".to_string()],
			function_name: Vec::new(),
			debug: false,
			verbose: false,
			path: ".".to_string()
		}
	}
}