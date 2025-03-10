/// Represents a task with title, description, and due date.
pub struct Task {
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub completed: bool,
}

impl Task {
    /// Creates a new task.
    pub fn new(title: &str, description: &str, due_date: &str) -> Self {
        Task {
            title: title.to_string(),
            description: description.to_string(),
            due_date: due_date.to_string(),
            completed: false,
        }
    }
}