/*-------------
/lib.rs

This file is for the connection to the server and tokens.
-------------*/
#[macro_use] extern crate quote;
#[macro_use] extern crate lazy_static;
use state::State;
use tracing::error;
use std::error::Error;
use std::fs;
use syn::visit::Visit;
use input::{toml, tree::{BTree, Node}};
use config::APP;

pub mod config;
pub mod file;
pub mod graph;
pub mod input;
pub mod log;
pub mod parse;
pub mod stack;
pub mod state;


/// Launch the application
/// This need a bit of a rework to make it more modular but im not done with the application yet so
/// this will do for now.
pub fn launch() -> Result<(), Box<dyn Error>> {
	// Grab Directory and files
    let mut root = Node::new();
    root.add_key(&APP.path);

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
            let content = fs::read_to_string(leaf.clone())?;
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
	
	Ok(())
}