use std::env;
use todo::{util, Config};

fn main() {
    let args = env::args();
    
    let config = Config::new(args).unwrap_or_else(|err| {
        panic!("Error creating config: {}", err);
    });

    util::toggle_file_lock(&config.output_file, false).unwrap();

    let action = config.parse_args().unwrap_or_else(|err| {
        panic!("Error parsing args: {}", err);
    });

    config.dispatch_action(action).unwrap_or_else(|err| {
        panic!("Error dispatching action: {}", err);
    });

    util::toggle_file_lock("todos.txt", true).unwrap();
}
