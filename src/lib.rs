use std::error::Error;
use std::io::prelude::*;
use std::fs::{OpenOptions};

enum Completion {
  Pending,
  Completed,
  Reopened,
}

impl Completion {
  fn to_string(&self) -> String {
    match self {
      Pending => String::from("pending"),
      Completed => String::from("completed"),
      Reopened => String::from("reopened"),
    }
  }
}

pub struct TodoItem {
  description: String,
  status: Completion,
}

impl TodoItem {
  pub fn new(description: &str) -> TodoItem {
    TodoItem {
      description: String::from(description),
      status: Completion::Pending,
    }
  }

  fn format(&self) -> String {
    format!("
description: {}
status:      {}
", self.description, self.status.to_string())
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {

    let mut file = OpenOptions::new()
      .append(true)
      .create(true)
      .open("todos.txt")?;

    file.write_all(self.format().as_bytes())?;

    Ok(())
  }
}
