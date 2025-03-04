use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    ara::launch()
}