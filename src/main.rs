use std::env;
use todo::todo_item::TodoItem;

fn main() {

    let args: Vec<String> = env::args().collect();

    let todo_item = TodoItem::new(&args[1]).unwrap_or_else(|err| {
        panic!("There was an error creating a TodoItem!: {}", err);
    });

    println!("{:?}", todo_item);
    
    todo_item.save().unwrap_or_else(|err| {
        panic!("There was an error!: {}", err);
    });

}
