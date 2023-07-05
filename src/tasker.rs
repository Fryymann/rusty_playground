#![allow(dead_code)]
#![crate_name = "tasker"]

/// A single task is repesented here
pub struct Task {
    /// A task must have a description
    pub description: String,
    /// A task must be given a priority
    pub priority: i32,
    /// A task is given false as its default value for 'complete'
    pub complete: bool,
}

impl Task {
    /// Returns a Task with the description and priority given
    ///
    /// # Arguments
    ///
    /// * `description` - A string that represents the actual task itself.
    /// * `priority` - A number that denotes the level of the task's importance
    pub fn new(description: String, priority: i32) -> Task {
        Task {
            description: description,
            priority: priority,
            complete: false,
        }
    }

    /// Prints its values
    ///
    /// Prints its `description` on a line, then it's `priority` on the next, and its `complete` boolean value
    pub fn print(&self) {
        println!("{}", &self.description);
        println!("Priority: {}", &self.priority);
        println!("Finished: {}", &self.complete);
    }

    /// Finish/Un-Finish a Task
    ///
    /// Inverts the values of a Task's `complete` boolean value on a third line
    pub fn toggle(&mut self) {
        self.complete = !self.complete;
    }
}

pub fn delete_task(task: Task) {
    println!("Deleting Task: {}...", task.description);
}

/// Represents a list of tasks
pub struct TaskList {
    /// A TaskList must have a name to distinguish itself from other TaskLists
    pub name: String,
    /// A TaskList is given a `Vector<Task>` to hold the individual Tasks associated with the list
    pub tasks: Vec<Task>,
}

impl TaskList {
    /// Returns a TaskList with the provided name and an empty Task Vector
    ///
    /// # Arguments
    ///
    /// * `name` - A StringSlice representing the name used to identify the list
    pub fn new(name: &str) -> TaskList {
        TaskList {
            name: name.to_string(),
            tasks: Vec::new(),
        }
    }

    /// Print Tasks belonging to the Tasklist
    ///
    /// Iterates over the Tasks within the TaskLists Vector and calls `print()` on each one
    pub fn print(&self)  {
        let mut index: i32 = 0;
        println!("\n-------------------------------");
        for task in &self.tasks {
            println!("Task {}:", index);
            task.print();
            index += 1;
            println!("-------------------------------");
        }
    }

    /// Add a Task to the TaskList
    ///
    /// Takes a String and an i32 and creates a Task which is then inserted into the TaskList's `Vector<Task>`
    pub fn add_task(&mut self, description: String, priority: i32) {
        let task = Task::new(description, priority);
        self.tasks.push(task);
    }

    /// Toggle a Task in the TaskList as complete or incomplete
    ///
    /// Takes an index value and toggles the boolean value of the Task residing at that index in the Tasklist
    pub fn toggle_task(&mut self, index: usize) {
        let task: &mut Task = &mut self.tasks[index];
        task.toggle();
    }

    pub fn sort(&mut self) {
        self.tasks.sort_by(|a: &Task, b: &Task| a.priority.cmp(&b.priority));
    }
}
