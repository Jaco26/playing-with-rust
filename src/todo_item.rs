extern crate uuid;

use std::error::Error;
use std::io::prelude::*;
use std::fs::{OpenOptions};
use uuid::Uuid;

#[derive(Debug)]
enum Completion {
  Pending,
  Complete,
  Reopened,
}

#[derive(Debug)]
pub struct TodoItem {
  id: Uuid,
  description: String,
  status: Completion,
}

impl TodoItem {
  pub fn new(description: &str) -> Result<TodoItem, &'static str> {
    Ok(TodoItem {
      id: Uuid::new_v4(),
      status: Completion::Pending,
      description: String::from(description),
    })
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
      .append(true)
      .create(true)
      .open("todos.txt")?;
    
    file.write_all(self.format_output().as_bytes())?;

    Ok(())
  }

  fn status_to_string(&self) -> String {
    match &self.status {
      Completion::Pending => String::from("pending"),
      Completion::Complete => String::from("complete"),
      Completion::Reopened => String::from("reopened"),
    }
  }

  fn format_output(&self) -> String {
    format!("
id:            {}
status:        {}
descrtiption:  {}", self.id, self.status_to_string(), self.description)
  }

  

}