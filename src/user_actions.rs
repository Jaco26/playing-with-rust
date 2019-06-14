use std::fs;
use std::io::{ Error };
use std::collections::HashMap;
use super::todo_item::TodoItem;


pub fn read(filepath: &str) -> Result<(), Error> {
  let file_str = fs::read_to_string(filepath)?;

  let items_txt_iter = file_str.split("\n\r");

  let items = items_txt_iter.fold(Vec::<TodoItem>::new(), |mut acc, item: &str| {
    let mut map: HashMap<String, String> = HashMap::new();

    for line in item.lines() {
      let segments: Vec<String> = line.split(":").map(|item| item.trim())
        .map(|s| s.to_string())
        .collect();

      if segments.len() > 1 {
        map.insert(segments[0].to_string(), segments[1].to_string());
      }
    }

    if !map.is_empty() {
      acc.push(TodoItem::from_hash_map(map));
    }
    
    acc
  });

  println!("{:#?}", items);

  Ok(())
}

pub fn write(filepath: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
  let item = TodoItem::new(body);

  item.save_to(filepath)?;

  Ok(())
}
