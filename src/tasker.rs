
    pub fn create_task(description: String, priority: u32) -> (String, u32, bool) {
        let task = (description, priority, false);
        task
    }

    pub fn print_task(task: &(String, u32, bool)) {
        println!("{} - {} : {}", task.0, task.1, task.2);
    }

    pub fn update_task(task: &mut (String, u32, bool)) {
        task.2 = !task.2;
    }

    pub fn delete_task(task: (String, u32, bool)) {
        println!("Deleting Task: {}...", task.0);
    }

    pub fn create_list() -> Vec<(String, u32, bool)> {
        let list: Vec<(String, u32, bool)> = Vec::new();
        list
    }

    pub fn print_list(list: &Vec<(String, u32, bool)>) {
        for task in list {
            print_task(&task);
        }
        println!("-------------------------------");
    }

    pub fn update_list(list: &mut Vec<(String, u32, bool)>, index: usize) {
        update_task(&mut list[index]);
    }

    pub fn add_task(list: &mut Vec<(String, u32, bool)>, new_task_description: String, priority: u32) {
        let task = create_task(new_task_description, priority);
        list.push(task);
        // prioritize_list(list);
    }

    pub fn delete_task_on_list(list: &mut Vec<(String, u32, bool)>, index: usize) {
        list.remove(index);
    }

    pub fn delete_list(list: Vec<(String, u32, bool)>) {
        println!("Deleting Task List...");
    }

    pub fn prioritize_list(list: &mut Vec<(String, u32, bool)>) {
        list.sort_by(|a, b| a.2.cmp(&b.2));
        println!("Prioritizing....");
    }

