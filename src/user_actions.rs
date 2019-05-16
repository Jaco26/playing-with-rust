use std::fs;
use std::io::{Error, ErrorKind};




fn read(filepath: &str) -> Result<(), Error> {
  let file_str = fs::read_to_string(filepath)?;
  let lines_vec: Vec<&str> = file_str.split("\n\n").collect();
  println!("{:#?}", lines_vec[0].trim());
  // println!("{}", file_vec);
  // for line in lines_vec {
  //   println!("{}", line.to_ascii_lowercase());
  // }
  Ok(())
}

pub struct ActionDispatcher;


impl ActionDispatcher {
  pub fn parse(&self, opt: &str) -> Result<(), Box<dyn std::error::Error>> {
    match opt {
      "-r" | "--read" => {
        read("todos.txt")?;
        Ok(())
      },
      "-w" | "--write" => {
        println!("writing!");
        Ok(())
      },
      "-u" | "--update" => {
        println!("updating!");
        Ok(())
      },
      "-d" | "--delete" => {
        println!("deleting!");
        Ok(())
      },
      _ => Err(Box::new(Error::new(ErrorKind::Other, "No matching flags")))
    }
  }
}

