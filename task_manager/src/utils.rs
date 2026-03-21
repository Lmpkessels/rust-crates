pub fn add_task(task: &str) {
    println!("{} added to file", task);
}

pub fn remove_task(task: &str) {
    println!("{} removed from file", task);
}

pub fn list_tasks() {
    println!("A list of all tasks");
}

pub fn mark_as_complete(task: &str) {
    println!("{} is marked as completed", task);
}