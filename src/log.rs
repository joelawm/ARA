use colored::Colorize;
use crate::config::APP;

pub mod info {
	use super::*;

	pub fn print_visit_function(function: &str, file: &str) {
		println!("{} {} {}{}", "Visiting Function:".blue().bold(), "fn".green(), function.green(), "()".green());
		println!("\t{} {}", "in".italic(), &file.italic());
	}
	
	pub fn print_loc(loc: &str) {
		if !APP.verbose {
			println!("LOC: {}", loc.green());
		}
	}
}

pub mod debug {
	use super::*;
	use std::fmt::Display;

	pub fn debug<T: std::fmt::Debug + Display>(msg: &T) {
		if APP.verbose {
			println!("{} {}", "Debug:".blue().bold(), msg);
		}
	}

	pub fn print_expr(loc: &str) {
		if APP.verbose {
			println!("LOC: {}", loc.green());
		}
	}

	pub fn warn<T: std::fmt::Debug + Display>(msg: &T) {
		if APP.verbose {
			println!("{} {}", "Warning:".yellow().bold(), msg);
		}
	}

	pub fn error<T: std::fmt::Debug + Display>(msg: &T) {
		println!("{} {}", "Error:".red().bold(), msg);
	}
}