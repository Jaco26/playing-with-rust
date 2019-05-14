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

impl Completion {
  fn to_string(&self) -> String {
    match *self {
      Completion::Reopened => String::from("reopened"),
      Completion::Complete => String::from("complete"),
      Completion::Pending => String::from("pending"),
    }
  }

  fn from_string(&self, name: &str) -> Option<Completion>{
    match name {
      "reponened" => Some(Completion::Reopened),
      "complete" => Some(Completion::Complete),
      "pending" => Some(Completion::Pending),
      _ => None
    }
  }
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

  pub fn save_to(&self, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
      .append(true)
      .create(true)
      .open(filename)?;
    
    file.write_all(self.format_output().as_bytes())?;

    Ok(())
  }

  fn format_output(&self) -> String {
    format!("

id:            {}
status:        {}
descrtiption:  {}", self.id, self.status.to_string(), self.description)
  }
}