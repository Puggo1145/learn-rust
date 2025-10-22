use std::error::Error;
use std::fs::{read_to_string};

// TODO: panic or not
fn main() -> Result<(), Box<dyn Error>> {
    read_to_string("hello.txt")?;
    Ok(())
}
