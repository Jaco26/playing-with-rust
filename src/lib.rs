pub mod user_actions;
pub mod util;
pub mod todo_item;

#[derive(Debug)]
pub enum Action {
  Create { body: String },
  Read,
}

pub struct Config {
  flag: String,
  pub output_file: String,
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

    Ok( Config { flag, output_file, body, action: None  } )
  }

  pub fn parse_args(&mut self) -> Result<(), &'static str> {
    match self.flag.as_str() {
      "-r" => self.action = Some(Action::Read),
      "-w" => {
        match &self.body {
          Some(txt) => self.action = Some(Action::Create { body: String::from(txt.as_str()) }),
          None => return Err("No body provided with -w action flag"),
        };
      },
      _ => return Err("No action flag present")
    };
    Ok(())
  }

  pub fn dispatch_action(&self) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(action) = &self.action {
      match action {
        Action::Read => user_actions::read(&self.output_file)?,
        Action::Create{ body } => {
          user_actions::write(&self.output_file, &body)?;
        },
      }
    };
    Ok(())
  }
}


