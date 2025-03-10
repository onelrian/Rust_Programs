mod task;
mod task_manager;

use task_manager::TaskManager;

fn main() {
    let mut task_manager = TaskManager::new();

    task_manager.add_task("Task 1".to_string(), "This is task 1".to_string(), "2023-03-01".to_string());
    task_manager.add_task("Task 2".to_string(), "This is task 2".to_string(), "2023-03-15".to_string());

    task_manager.display_tasks();

    task_manager.update_task("Task 1", Some("This is updated task 1".to_string()), None);

    task_manager.display_tasks();

    task_manager.remove_task("Task 2");

    task_manager.display_tasks();
}