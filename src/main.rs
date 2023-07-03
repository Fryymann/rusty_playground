fn create_task(description: String) -> (String, bool) {
    let task = (description, false);
    task
}

fn print_task(task: &(String, bool)) {
    println!("{} - {}", task.0, task.1);
}
// fn get_task_string(task: &(String, bool)) -> String {

// }

fn update_task(task: &mut (String, bool)) {
    task.1 = !task.1;
}

fn delete_task(task: (String, bool)) {
    println!("Deleting Task: {}...", task.0);
}


fn create_list() -> Vec<(String, bool)> {
    let list: Vec<(String, bool)>= Vec::new();
    list
}

fn print_list(list: &Vec<(String, bool)>) {
    for task in list {
        print_task(&task);
    }
    println!("-------------------------------");
}

fn update_list(list: &mut Vec<(String, bool)>, index: usize) {
    update_task(&mut list[index]);
}


fn add_task(list: &mut Vec<(String, bool)>, new_task_description: String) {
    let task = create_task(new_task_description);
    list.push(task);
}

fn delete_task_on_list(list: &mut Vec<(String, bool)>, index: usize) {
    list.remove(index);
}

fn delete_list(list: Vec<(String, bool)>) {
    println!("Deleting Task List...");
}



fn main() {
    let mut task_list = create_list();
    add_task(&mut task_list, "this new thing".to_string());
    add_task(&mut task_list, "this other thing".to_string());
    print_list( &task_list );
    update_list(&mut task_list, 0);
    print_list( &task_list );

    delete_task_on_list(&mut task_list, 1);
    print_list( &task_list );

}