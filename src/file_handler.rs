use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn append_to(filename: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(filename)?;
  
  file.write_all(body.as_bytes())?;

  Ok(())
}

pub fn write_new(filename: &str, body: &str) ->  Result<(), Box<dyn std::error::Error>> {
  let mut file = OpenOptions::new()
    .create_new(true)
    .open(filename)?;
  
  file.write_all(body.as_bytes())?;

  Ok(())
}