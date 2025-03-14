/*-------------
/lib.rs

This file is for the connection to the server and tokens.
-------------*/
#[macro_use] extern crate quote;
use error::Error;
use state::State;
use tracing::error;
use std::fs;
use syn::visit::Visit;
use input::{toml, tree::{BTree, Node}};
use config::{Settings, APP};

pub mod config;
pub mod error;
pub mod file;
pub mod graph;
pub mod input;
pub mod log;
pub mod parse;
pub mod stack;
pub mod state;

pub struct Ara {
    pub state: State,
    settings: Settings
}

impl Ara {
    pub fn new() -> Self {
        Ara {
            state: State::new(),
            settings: Settings::new()
        }
    }
    /// Launch the application
    pub fn launch(&mut self) -> Result<&Self, Error> {
        // Set Settings
        if let Err(e) = APP.set(self.settings.clone()) {
            error!("{:?}", e);
            return Err(Error::NewSettingsError("Error setting settings".to_string()));
        }

        // Grab Directory and files
        let mut root = Node::new();
        root.add_key(&APP.get().unwrap().path);

        let mut directories = BTree::new(Some(root));
        file::grab_files(&mut directories.get_root().as_mut().unwrap());

        //log::debug::debug(&directories);

        // Grab Workspace
        let workspace = toml::parse_toml();

        // Create a new Graph
        let mut visitor = State::new();
        workspace.workspace.members.iter().for_each(|member| {
            visitor.add_workspace_lib(member.clone());
        });

        for leaf in directories.get_all_leafs() {
            if leaf.ends_with(".rs") {
                let content = match fs::read_to_string(leaf.clone()) {
                    Ok(content) => content,
                    Err(err) => {
                        error!("Error reading file: {}", err);
                        continue;
                    }
                };
                let syntax = match syn::parse_file(&content) {
                    Ok(syntax) => syntax,
                    Err(err) => {
                        error!("Error parsing file: {}", leaf);
                        error!("{}", err);
                        continue;
                    }
                };
                visitor.update_current_file(leaf);
                visitor.visit_file(&syntax);
                visitor.clear_libs();
            }
        }
        self.state = visitor;
        
        Ok(self)
    }

    /// Set the current file
	pub fn ignore(&mut self, ignore: &Vec<String>) -> &mut Self {
		self.settings.ignore = ignore.to_owned();
        self
	}

	/// Set the function name
	pub fn function_name(&mut self, function_name: &Vec<String>) -> &mut Self {
		self.settings.function_name = function_name.to_owned();
        self
	}

	/// Set the debug mode
	pub fn debug(&mut self, debug: bool) -> &mut Self {
		self.settings.debug = debug;
        self
	}

	/// Set the verbose mode
	pub fn verbose(&mut self, verbose: bool) -> &mut Self {
		self.settings.verbose = verbose;
        self
	}

	/// Set the path
	pub fn path(&mut self, path: &str) -> &mut Self {
		self.settings.path = path.to_string();
        self
	}
}
