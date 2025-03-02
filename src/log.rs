pub mod info {
	use colored::Colorize;

	pub fn print_visit_function(function: &str, file: &str) {
		println!("{} {} {}{}", "Visiting Function:".blue().bold(), "fn".green(), function.green(), "()".green());
		println!("\t{} {}", "in".italic(), &file.italic());
	}
	
	pub fn print_loc(loc: &str) {
		println!("LOC: {}", loc.green());
	}
}

pub mod debug {
	use colored::Colorize;
	use crate::config::APP;

	pub fn print_expr(loc: &str) {
		if APP.verbose {
			println!("LOC: {}", loc.green());
		}
	}

	pub fn warn(msg: &str) {
		if APP.verbose {
			println!("{} {}", "Warning:".yellow().bold(), msg.yellow());
		}
	}
}