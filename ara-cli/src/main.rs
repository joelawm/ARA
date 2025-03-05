#[macro_use] extern crate lazy_static;
use std::error::Error;

mod config;

pub fn main() -> Result<(), Box<dyn Error>> {
    ara::launch()
}