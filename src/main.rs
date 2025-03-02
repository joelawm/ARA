#[macro_use] extern crate quote;
#[macro_use] extern crate lazy_static;
use state::State;
use tracing::error;
use std::error::Error;
use std::fs;
use syn::visit::Visit;
use input::{toml, tree::{BTree, Node}};
use config::APP;

mod config;
mod graph;
mod input;
mod log;
mod parse;
mod stack;
mod state;

pub fn main() -> Result<(), Box<dyn Error>> {
    // Grab Directory and files
    let mut root = Node::new();
    root.add_key(&APP.path);

    let mut directories = BTree::new(Some(root));
    grab_files(&mut directories.get_root().as_mut().unwrap());

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

/// Grab all the files in the directory
pub fn grab_files(directory: &mut Node) {
    let paths = fs::read_dir(directory.to_string()).unwrap();

    for path in paths {
        let is_dir = path.as_ref().unwrap().path().is_dir();
        let path = path.unwrap().path().to_str().unwrap().to_string();

        if config::APP.ignore.iter().any(|ignore| path.contains(ignore)) {
            continue;
        }

        if is_dir {
            let mut node = Node::new();
            node.add_key(&path);
            grab_files(&mut node);
            directory.add_child(node);
        } else {
            directory.add_key(&path);
        }
    }
}
