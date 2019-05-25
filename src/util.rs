use std::{fs, io::Error};

pub fn toggle_file_lock(filepath: &str, lock_file: bool) -> Result<(), Error> {
    let mut perms = fs::metadata(filepath)?.permissions();
    perms.set_readonly(lock_file);
    fs::set_permissions(filepath, perms)?;
    Ok(())
}