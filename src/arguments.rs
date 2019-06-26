

struct Arg {
  pattern: String,
  action: Box<Fn()>,
}

pub struct Arguments {
  args: Vec<Arg>,
}

impl Arguments {
  pub fn new() -> Arguments {
    Arguments {
      args: vec![],
    }
  }
  pub fn add_arg(&mut self, pattern: &str, action: Box<Fn()>) {
    self.args.push(Arg { 
      pattern: String::from(pattern),
      action,
    });
  }
  pub fn parse_args(args: std::env::Args) {

  }
}
