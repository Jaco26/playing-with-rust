
use std::fs;
use std::io::{ Error };
use std::collections::HashMap;
use crate::todo_item::{TodoItem, Completion};
use crate::util::file_handler;


pub fn read(filepath: &str) -> Result<HashMap<String, TodoItem>, Error> {
  let file_text = fs::read_to_string(filepath)?;

  let todos_map: HashMap<String, TodoItem> = file_text.split("^#")
    .fold(HashMap::<String, TodoItem>::new(), |mut acc, section| {
      if !section.is_empty() {
        let mut id = "";
        let mut status = "";
        let mut description = "";
        for line in section.lines() {
          let mut segments = line.split(":");
          let key = segments.next().unwrap_or("");
          let value = segments.next().unwrap_or("");
          match key {
            "id" => id = value,
            "status" => status = value,
            "description" => description = value,
            _ => (),
          }
        }
        acc.insert(String::from(id), TodoItem::from_args(id, status, description));
      }
      acc
    });

  Ok(todos_map)
}

pub fn write(filepath: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
  let item = TodoItem::new(body);

  let formatted_text = item.format_dense();

  file_handler::append_to(filepath, formatted_text.as_str())?;

  Ok(())
}

pub fn update_body(filepath: &str, id: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut todos_map = read(filepath)?;

  let to_update = match todos_map.get_mut(id) {
    Some(item) => item,
    None => panic!("Could not find item with id = {}", id),
  };

  to_update.description = String::from(body);

  let status = match &to_update.status {
    Some(val) => val.to_string(),
    None => String::from(""),
  };

  let updated = TodoItem::from_args(id, status.as_str(), body);

  todos_map.insert(String::from(id), updated).unwrap();

  println!("Updated Todos HashMap: {:#?}", todos_map);

  Ok(())
}
