mod user_actions;
mod todo_item;
mod arguments;
pub mod util;

use util::file_handler;

#[derive(Debug)]
pub enum Action {
  Create { body: String },
  Update { id: String, body: String }, 
  Read,
}

pub struct Config {
  flag: String,
  pub output_file: String,
  id: Option<String>,
  body: Option<String>,
  action: Option<Action>,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next(); // consume the executable name...the first command line arg

    let output_file = String::from("todos.txt");

    let flag = match args.next() {
      Some(s) => s,
      None => return Err("No action flag provided"),
    };

    let body = args.next();

    let id = args.next();


    Ok( Config { flag, output_file, body, id, action: None } )
  }

  pub fn parse_args(&mut self) -> Result<(), &'static str> {
    match self.flag.as_str() {
      "-r" => self.action = Some(Action::Read),
      "-w" => {
        match &self.body {
          Some(txt) => self.action = Some(Action::Create { body: txt.to_string() }),
          None => return Err("No body provided with -w action flag"),
        };
      },
      "-u" => {
        match &self.body {
          Some(txt) => {
            match &self.id {
              Some(val) => self.action = Some(Action::Update { id: val.to_string(), body: txt.to_string() }),
              None => panic!("No Id provided for update"),
            };
          },
          None => return Err("No body provided with -w action flag"),
        };
      }
      _ => return Err("No action flag present")
    };
    Ok(())
  }

  pub fn dispatch_action(&self) -> Result<(), Box<dyn std::error::Error>> {

    file_handler::toggle_file_lock(&self.output_file, false)?;

    if let Some(action) = &self.action {
      match action {
        Action::Read => {
          let todos_map = user_actions::read(&self.output_file)?;
          println!("{:#?}", todos_map);
        },

        Action::Create{ body } => {
          user_actions::write(&self.output_file, &body)?;
        },

        Action::Update { id, body } => {
          user_actions::update_body(&self.output_file, id, body)?;
        }
      }
    };

    file_handler::toggle_file_lock(&self.output_file, true)?;

    Ok(())
  }
}


