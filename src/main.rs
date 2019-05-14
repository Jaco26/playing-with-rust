use std::{fs, env, io::Error};
use todo::todo_item::TodoItem;
use todo::user_options;

fn main() {

    toggle_file_lock("todos.txt", false).unwrap();

    let args: Vec<String> = env::args().collect();

    let todo_item = TodoItem::new(&args[1]).unwrap_or_else(|err| {
        panic!("There was an error creating a TodoItem!: {}", err);
    });

    todo_item.save_to("todos.txt").unwrap_or_else(|err| {
        panic!("There was an error saving the TodoItem!: {}", err);
    });

    toggle_file_lock("todos.txt", true).unwrap();

}


fn toggle_file_lock(filepath: &str, lock_file: bool) -> Result<(), Error> {
    let mut perms = fs::metadata(filepath)?.permissions();
    perms.set_readonly(lock_file);
    fs::set_permissions(filepath, perms)?;
    Ok(())
}
