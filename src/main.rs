use std::env;
use todo::Config;

fn main() {

    let args = env::args();
    
    let mut config = Config::new(args).unwrap_or_else(|err| {
        panic!("Error creating config: {}", err);
    });

    if let Err(err) = config.parse_args() {
        panic!("Error parsing args: {}", err);
    }

    config.dispatch_action().unwrap_or_else(|err| {
        panic!("Error dispatching action: {}", err);
    });
}
