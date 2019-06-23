use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;

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
    .write(true)
    .create_new(true)
    .open(filename)?;
  
  file.write_all(body.as_bytes())?;

  Ok(())
}


pub fn toggle_file_lock(filepath: &str, lock_file: bool) -> Result<(), Error> {
    let mut perms = fs::metadata(filepath)?.permissions();
    perms.set_readonly(lock_file);
    fs::set_permissions(filepath, perms)?;
    Ok(())
}
