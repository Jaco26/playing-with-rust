
use std::fs;
use std::io::{ Error };
use std::collections::HashMap;
use super::todo_item::TodoItem;
use super::file_handler;


pub fn read(filepath: &str) -> Result<(), Error> {
  let file_text = fs::read_to_string(filepath)?;

  let todos_map: HashMap<String, TodoItem> = file_text.split("\n\r")
    .fold(HashMap::<String, TodoItem>::new(), |mut acc, section| {
      if !section.trim().is_empty() {
        let todo_item = TodoItem::from_text(section);
        acc.insert(todo_item.id.to_string(), todo_item);
      }
      acc
    });
  
  println!("{:#?}", todos_map);

  Ok(())
}

pub fn write(filepath: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
  let item = TodoItem::new(body);

  let formatted_text = item.format_dense();

  file_handler::append_to(filepath, formatted_text.as_str())?;

  Ok(())
}
