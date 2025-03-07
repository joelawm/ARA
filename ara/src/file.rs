/*-------------
/file.rs

This file is for the handling the creation of the file tree for the application.
-------------*/
use std::fs;
use tracing::error;
use crate::{config, input::tree::Node};

/// Grab all the files in the directory
pub fn grab_files(directory: &mut Node) {
    let paths = match fs::read_dir(directory.to_string()) {
        Ok(paths) => paths,
        Err(err) => {
            error!("Error reading directory: {}", err);
            return;
        }
    };

    for path in paths {
        let is_dir = match path.as_ref() {
            Ok(path) => path.path().is_dir(),
            Err(err) => {
                error!("Error seeing if path is a dir: {}", err);
                continue;
            }
        };

        let path = match path {
            Ok(path) => path.path().to_string_lossy().to_string(),
            Err(err) => {
                error!("Error reading path: {}", err);
                continue;
            }
        };

        if config::APP.get().unwrap().ignore.iter().any(|ignore| path.contains(ignore)) {
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
