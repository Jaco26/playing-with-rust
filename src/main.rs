use std::env;
use todo::{
    util,
    user_actions, 
    todo_item, 
    Config,
};

fn main() {

    let action_dispatcher = user_actions::ActionDispatcher;
    
    util::toggle_file_lock("todos.txt", false).unwrap();

    let args = env::args();

    let item = todo_item::TodoItem::new(args).unwrap_or_else(|err| {
        panic!("There was an error creating a TodoItem!: {}", err);
    });

    item.save_to("todos.txt").unwrap_or_else(|err| {
        panic!("There was an error saving the TodoItem!: {}", err);
    });

    util::toggle_file_lock("todos.txt", true).unwrap();

}


