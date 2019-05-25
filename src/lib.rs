pub mod user_actions;
pub mod util;
pub mod todo_item;

pub struct Config {
  flag: String,
  body: Option<String>
}

impl Config {
  pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
    if args.len() < 1 || args.len() > 2 {
      return Err("Incorrect number of args");
    }

    let flag = args[0].clone();
    let body;

    if args.len() == 1 {
      body = None;
    } else {
      body = Some(args[1].clone());
    }

    Ok( Config { flag, body } )
  }
}
