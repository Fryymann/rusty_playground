use std::io;
mod tasker;

fn main() {
    let mut task_list = tasker::create_list();

    loop {
        println!("Enter a command: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command: &str = input.trim();

        if command == "add" {
            let mut new_input = String::new();
            println!("Enter a task description: ");
            io::stdin().read_line(&mut new_input).unwrap();
            let description = input.trim();

            println!("Enter the task's priority: ");
            io::stdin().read_line(&mut new_input).unwrap();
            let priority = input.trim().to_string().parse::<u32>().unwrap();
            // command = input.trim();
            tasker::add_task(&mut task_list, description.to_string(), priority);
        }

        if command == "quit" {
            break;
        }
    }

    // tasker::add_task(&mut task_list, "this new thing".to_string(), 3);
    // tasker::print_list(&task_list);

    // tasker::add_task(&mut task_list, "this other thing".to_string(), 1);
    // tasker::print_list(&task_list);

    // tasker::update_list(&mut task_list, 0);

    // tasker::prioritize_list(&mut task_list);
    // tasker::print_list(&task_list);

    //  // Print a message asking for user input
    //  println!("Please enter your name:");

    //  // Create a mutable string to store the user's input
    //  let mut input = String::new();

    //  // Read the user's input and store it in the 'input' string
    //  io::stdin().read_line(&mut input).expect("Failed to read line");

    //  // Trim any leading/trailing whitespaces or newline characters
    //  let name = input.trim();

    //  // Print the user's input
    //  println!("Hello, {}!", name);

    // Example input string
    let input_str = "42";

    // Parse the string to u32 using the parse method
    match input_str.parse::<u32>() {
        Ok(number) => {
            // The parsing was successful, 'number' is of type u32
            println!("Parsed value: {}", number);
        }
        Err(_) => {
            // The parsing failed, handle the error here
            println!("Failed to parse the input as u32.");
        }
    }
}
