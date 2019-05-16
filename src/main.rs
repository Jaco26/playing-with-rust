use std::{fs, env, io::Error};
// use todo::todo_item::TodoItem;
use todo::user_actions;

fn main() {

    let action_dispatcher = user_actions::ActionDispatcher;

    

    // toggle_file_lock("todos.txt", false).unwrap();

    let args: Vec<String> = env::args().collect();

    action_dispatcher.parse("-r").unwrap_or_else(|err| {
        panic!("Error dispatching action!: {}", err)
    });

    // let todo_item = TodoItem::new(&args[1]).unwrap_or_else(|err| {
    //     panic!("There was an error creating a TodoItem!: {}", err);
    // });

    // todo_item.save_to("todos.txt").unwrap_or_else(|err| {
    //     panic!("There was an error saving the TodoItem!: {}", err);
    // });

    // toggle_file_lock("todos.txt", true).unwrap();

}


// fn toggle_file_lock(filepath: &str, lock_file: bool) -> Result<(), Error> {
//     let mut perms = fs::metadata(filepath)?.permissions();
//     perms.set_readonly(lock_file);
//     fs::set_permissions(filepath, perms)?;
//     Ok(())
// }
