use std::{env, process};

use todo;

fn main() {

    todo::hello_there();

    let args: Vec<String> = env::args().collect();

    let arg_string: String = args[1..].iter().fold("".to_string(), |acc, x| {
        acc + " " + x
    });

    println!("{}", arg_string);
    
    // for arg in args {
    //     println!("{}", arg);
    // }

}
