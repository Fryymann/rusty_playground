


/**
 *
 * Todo - struct that represents an individual task
 *
 * TodoList - struct that manages Todos.
 *
 *
 */
#[derive(Debug)]
struct Todo {
    description: String,
    complete: bool,
}

impl Todo {
    fn complete_task(&mut self) {
        self.complete = true;
    }


    fn print(&self) {
        println!("Description: {description} - Completed: {complete}", description = self.description, complete = self.complete);
    }
}

struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {

    fn add_todo(&mut self, desc: String) {
        let new_todo = Todo{ description: desc, complete: false };
        self.list.push(new_todo);
    }

    fn print_list(&self) {
        for todo in &self.list {
            todo.print();
        }
    }


    fn complete_task(&mut self, index: usize) {
        self.list[index].complete_task();
    }

    fn clear_completed_todos(&mut self) {
        self.list = self.list.iter()
            .filter(|tasks| tasks.complete)
            .collect();
    }
}

fn main(){

    let mut task_list = TodoList{ list: Vec::new() };

    task_list.add_todo( "Water the dogs".to_string() );
    task_list.add_todo( "Learn structs in Rust".to_string() );

    task_list.complete_task(1);
    task_list.print_list();

    task_list.clear_completed_todos();

    task_list.print_list();
}

// let mut some_vec = vec![0, 10, 20, 30];
// if let Some(index) = some_vec.iter().find(|value| **value == 10) {
//     some_vec.swap_remove(*index);
// }

// if let Some(index) = some_vec.iter().position(|value| *value == 10) {
//     some_vec.swap_remove(index);
// }