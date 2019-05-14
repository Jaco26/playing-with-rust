use std::io::Error;
use std::fs;


pub enum UserOptions {
  Read,
  Write,
  Update,
  Delete,
}


impl UserOptions {
  fn from_string(&self, option_name: &str) -> Option<UserOptions> {
    match option_name {
      "-r" | "--read" => Some(UserOptions::Read),
      "-w" | "--write" => Some(UserOptions::Write),
      "-u" | "--update" => Some(UserOptions::Update),
      "-d" | "--delete" => Some(UserOptions::Delete),
      _ => None,
    }
  }

  fn read(filepath: &str) -> Result<(), Error> {
    let file_vec = fs::read(filepath)?;
    for line in file_vec {
      println!("{}", line);
    }
    Ok(())
  }

}
