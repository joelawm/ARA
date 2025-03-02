use std::fmt::Debug;
use crate::config::APP;
use crate::graph::Graph;
use crate::stack::Stack;

#[derive(Debug)]
pub struct State {
	pub graph: Graph,
	pub workspace_libs: Vec<String>,
	pub current_file: String,
	pub current_libs: Vec<String>,
	pub use_stack: Stack,
}

impl State {
	/// Create the state of the application
	pub fn new() -> State {
		State {
			graph: Graph::new(),
			workspace_libs: Vec::new(),
			current_libs: Vec::new(),
			current_file: String::new(),
			use_stack: Stack::new(),
		}
	}

	///
	pub fn add_new_lib(&mut self, lib: &str) {
		self.current_libs.push(self.use_stack.get_with_last(&lib));
	}

	///
	pub fn clear_libs(&mut self) {
		self.current_libs.clear();
	}

	///
	pub fn add_workspace_lib(&mut self, lib: String) {
		self.workspace_libs.push(lib);
	}

	///
	pub fn update_current_file(&mut self, file: String) {
		let file = file.replace(&APP.path, "").replace(".rs", "").replace("/src/", "").replace("/", "").replace("mod", "");
		self.current_file = file;
	}
}