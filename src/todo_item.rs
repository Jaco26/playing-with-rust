extern crate uuid;

use std::error::Error;
use std::io::prelude::*;
use std::fs::{OpenOptions};
use uuid::Uuid;

#[derive(Debug)]
pub enum Completion {
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

  fn empty() -> TodoItem {
    TodoItem {
      id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
      description: String::from(""),
      status: None,
    }
  }

  pub fn from_text(text: &str) -> TodoItem {
    let mut todo = TodoItem::empty();
    for line in text.lines() {
      let mut segments = line.split(":").map(|seg| seg.trim());
      let key = segments.next().unwrap_or("");
      let value = segments.next().unwrap_or("");
      match key {
        "id" => todo.id = Uuid::parse_str(value).unwrap_or_default(),
        "description" => todo.description = String::from(value),
        "status" => todo.status = Completion::from_str(value),
        _ => {},
      }
    }
    todo
  }

  pub fn format_dense(&self) -> String {
    let status = match &self.status {
      Some(val) => val.to_string(),
      None => panic!("Could not convert TodoItem.status to String")
    };
    format!("^#id:\"{}\",status:\"{}\",description:\"{}\"#$", self.id, status, self.description)
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