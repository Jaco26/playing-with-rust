use std::error::Error;
use std::io::prelude::*;
use std::fs::{OpenOptions};

pub fn write_to_file(msg: &str, filename: &str) -> Result<(), Box<dyn Error>> {
  
  let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(filename)?;

  file.write_all(msg.as_bytes())?;

  Ok(())
}