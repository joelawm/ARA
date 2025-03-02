// Import the required dependencies.
use serde_derive::Deserialize;
use tracing::{error, warn};
use std::fs;
use std::process::exit;
use toml;
use crate::config::APP;

// Top level struct to hold the TOML data.
#[derive(Deserialize, Debug)]
pub struct Data {
   pub workspace: Workspace,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize, Debug)]
pub struct Workspace {
    pub members: Vec<String>,
}

/// This parses the toml file and returns data about what the workspace is/isn't.
pub fn parse_toml() -> Data {
    let filename = format!("{}/Cargo.toml", APP.path);

    let contents = match fs::read_to_string(&filename) {
        Ok(c) => c,
        Err(_) => {
            error!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    if !contents.contains("workspace") {
        warn!("No workspace found in `{}`", filename);
        return Data {
            workspace: Workspace {
                members: vec!["axum".to_string()],
            },
        };
    }

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            error!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };
    data
}