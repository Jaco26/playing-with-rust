use std::env;
use todo;

fn main() {

    let args: Vec<String> = env::args().collect();

    let arg_string = args[1..].iter().fold("".to_string(), |acc, x| {
        acc + " " + x
    });

    if let Err(err) = todo::write_to_file(&arg_string, "todos.txt") {
        println!("Uh oh!: {}", err);
    }
}
