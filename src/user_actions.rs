use std::fs;
use std::io::{ Error };
use super::todo_item::TodoItem;


pub fn read(filepath: &str) -> Result<(), Error> {
  let file_str = fs::read_to_string(filepath)?;
  let lines_vec: Vec<&str> = file_str.split("\n\r").collect();
  
  println!("{:#?}", lines_vec);

  Ok(())
}

pub fn write(filepath: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
  let item = TodoItem::new(body);

  let result = item.save_to(filepath)?;

  Ok(())
}
