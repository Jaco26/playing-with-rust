extern crate uuid;

use uuid::Uuid;

#[derive(Debug)]
pub enum Completion {
  Pending,
  Complete,
  Reopened,
}

impl Completion {
  pub fn to_string(&self) -> String {
    match *self {
      Completion::Reopened => String::from("reopened"),
      Completion::Complete => String::from("complete"),
      Completion::Pending => String::from("pending"),
    }
  }

  fn from_str(name: &str) -> Option<Completion>{
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
  pub id: Uuid,
  pub description: String,
  pub status: Option<Completion>,
}

impl TodoItem {
  pub fn new(description: &str) -> TodoItem {
    TodoItem {
      id: Uuid::new_v4(),
      status: Some(Completion::Pending),
      description: String::from(description),
    }
  }

  pub fn from_args(id: &str, status: &str, description: &str) -> TodoItem {
    TodoItem {
      id: Uuid::parse_str(id).unwrap_or_default(),
      status: Completion::from_str(status),
      description: String::from(description),
    }
  }

  pub fn format_dense(&self) -> String {
    let status = match &self.status {
      Some(val) => val.to_string(),
      None => panic!("Could not convert TodoItem.status to String")
    };
    format!("^#id:{}\nstatus:{}\ndescription:{}\n", self.id, status, self.description)
  }

  pub fn format_pretty(&self) -> String {
    let status = match &self.status {
      Some(val) => val.to_string(),
      None => panic!("Could not convert TodoItem.status to String")
    };
    format!("
id:            {}
status:        {}
description:   {}\n\r", self.id, status, self.description)
  }
}