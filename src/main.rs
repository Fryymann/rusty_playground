use std::io;
mod db;
mod tasker;
use tasker::TaskList;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};


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


#[tokio::main]
async fn main() -> mongodb::error::Result<()>  {
    println!("Buiilding connection to MongoDB Atlas server...");
    let uri = "mongodb+srv://ideans:code1@rusty1.znwsgiu.mongodb.net/";
    let mut client_options = ClientOptions::parse(uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    println!("    Attempting connection to MongoDB...");
    let client = Client::with_options(client_options)?;
    println!("    Connection established. Client created.");

    println!("Attempting ping command on database 'Rusty'");
    client
        .database("rusty")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(())
}
