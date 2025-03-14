#[macro_use] extern crate lazy_static;
use ara::Ara;

mod config;

pub fn main() -> Result<(), ara::error::Error> {
    match Ara::new()
        .debug(config::APP.debug)
        .function_name(&config::APP.function_name)
        .ignore(&config::APP.ignore)
        .path(&config::APP.path)
        .verbose(config::APP.verbose)
        .launch() {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}