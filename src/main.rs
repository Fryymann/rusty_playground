use std::io;
mod db;
mod tasker;
use mongodb::{
    bson::doc,
    Client,
    Database,
};
use tasker::TaskList;

fn get_input() -> String {
    let mut r_input: String = String::new();
    io::stdin().read_line(&mut r_input).unwrap();
    let input = r_input.trim();
    String::from(input)
}

fn task_manager() {
    let mut task_list: TaskList = TaskList::new("Test List");

    loop {
        println!("\n Commands: list, add, toggle, sort, help, quit");
        println!("Enter a command: ");
        let command = get_input();

        if command == "add" {
            println!("\nEnter a task description: ");
            let description = get_input();

            println!("\nEnter the task's priority: ");
            let priority_str = get_input();
            let int_priority = priority_str.parse::<i32>().expect("Shit went wrong...");

            task_list.add_task(description, int_priority);
        }

        if command == "list" {
            task_list.print();
        }

        if command == "toggle" {
            println!("\nToggle task at which index?");
            let r_index = get_input();
            let index = r_index
                .parse::<usize>()
                .expect("Problem with creating index for update function.");
            task_list.toggle_task(index);
        }

        if command == "sort" {
            println!("Sorting tasks based on priority...");
            task_list.sort();
            task_list.print();
        }

        if command == "help" {
            println!("Valid commands:");
            println!("list, add, toggle, sort, print, quit\n");
        }

        if command == "quit" {
            break;
        }
    }
}

// #[tokio::main]
// async fn do_db_stuff() -> mongodb::error::Result<Database> {
//     let db = db::connect_db().unwrap();

//     println!("Collections in Database:");
//     for collection_name in db.list_collection_names(None).await? {
//         println!("{}", collection_name);
//     }

//     Ok(Database)
// }

fn main() {
    let result = db::connect_db_sync();
    // loop {
    //     let command = get_input();
    // }
}
