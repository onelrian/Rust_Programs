use std::collections::HashMap;
use crate::task::Task;

/// Manages tasks, including adding, removing, and updating.
pub struct TaskManager {
    tasks: HashMap<String, Task>,
}

impl TaskManager {
    /// Creates a new task manager.
    pub fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
        }
    }

    /// Adds a new task to the task manager.
    pub fn add_task(&mut self, title: String, description: String, due_date: String) {
        let task = Task::new(&title, &description, &due_date);
        self.tasks.insert(title, task);
    }

    /// Removes a task from the task manager.
    pub fn remove_task(&mut self, title: &str) {
        self.tasks.remove(title);
    }

    /// Updates a task in the task manager.
    pub fn update_task(&mut self, title: &str, description: Option<String>, due_date: Option<String>) {
        if let Some(task) = self.tasks.get_mut(title) {
            if let Some(description) = description {
                task.description = description;
            }
            if let Some(due_date) = due_date {
                task.due_date = due_date;
            }
        }
    }

    /// Displays a list of all tasks, including their status.
    pub fn display_tasks(&self) {
        for (title, task) in &self.tasks {
            let status = if task.completed { "Completed" } else { "Pending" };
            println!("{}: {} (Due: {}, Status: {})", title, task.description, task.due_date, status);
        }
    }
}