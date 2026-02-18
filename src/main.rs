use std::io::{stdin, stdout, Write};

#[derive(Clone)]
struct Task {
    name: String,
    completed: bool,
}

#[derive(Clone, Default)]
struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    pub fn new() -> Self {
        Self::default()
    }

    fn add_todo(&mut self, name: String) {
        // firstly check if the same name already exists
        if self.tasks.iter().any(|task| task.name == name) {
            println!("Task with name '{}' already exists!", name);
            return;
        }

        self.tasks.push(Task {
            name,
            completed: false,
        });
        println!("Created a new task with name: {}", self.tasks.last().unwrap().name)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn get_status(&self, name: &str) -> Option<bool> {
        self.tasks
            .iter()
            .find(|task| task.name == name)
            .map(|task| task.completed)
    }

    fn print_list(&self) {
        let total_tasks = self.tasks.len();
        println!("Found {} task(s)", total_tasks);
        for task in &self.tasks {
            let status = if task.completed { "✓" } else { "✗" };
            println!("[{}] {}", status, task.name)
        }
    }

    fn delete(&mut self, name: &str) {
        let total_before = self.tasks.len();
        self.tasks.retain(|task| task.name != name);
        let total_after = self.tasks.len();
        if total_after < total_before {
            println!("Deleted '{}'", name);
        } else {
            println!("Cannot find task '{}'", name);
        }
    }

    fn toggle(&mut self, name: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.name == name) {
            task.completed = !task.completed;
            let status = if task.completed { "completed" } else { "incomplete" };
            println!("Marked '{}' as {}", name, status);
        } else {
            println!("Cannot find task '{}'", name);
        }
    }

    fn clear(&mut self) {
        let total_before = self.tasks.len();
        self.tasks.retain(|task| !task.completed);
        let total_after = self.tasks.len();
        let diff = total_before - total_after;
        println!("Cleared {} completed task(s)", diff);
    }

    fn reset(&mut self) {
        self.tasks.clear();
        println!("All tasks have been deleted");
    }
}

struct Input {
    command: String,
    arg: String,
}

fn get_input() -> Input {
    let _ = stdout().flush();
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    // get the first space to divide as command and arg
    match s.find(' ') {
        None => Input {
            command: s,
            arg: "".to_string(),
        },
        Some(idx) => {
            let arg = s.split_off(idx).trim().to_string();
            Input {
                command: s.trim().to_string(),
                arg,
            }
        }
    }
}

fn main() {
    let mut todo = Todo::new();

    println!(
        r#"
Welcome to Rust TODO, run these commands
- new: Create new task
    example: new learn rust
- list: See all current tasks
- done: Mark a task as completed
    example: done learn rust
- clear: Clear all completed tasks
- reset: Clear all tasks
- remove: Remove a task
    example: remove learn rust
- exit: Exit the app
    "#
    );

    loop {
        print!("# ");
        let s = get_input();

        match s.command.as_str() {
            "new" => {
                if s.arg.is_empty() {
                    println!("Missing todo name!")
                } else {
                    todo.add_todo(s.arg);
                }
            }
            "list" => todo.print_list(),
            "done" => todo.toggle(&s.arg),
            "remove" => todo.delete(&s.arg),
            "reset" => todo.reset(),
            "clear" => todo.clear(),
            "exit" => {
                println!("Terminating the Rust TODO");
                break;
            }
            "" => {} // ignore empty command
            _ => println!("Unknown command: '{}'. Type 'list' to see commands.", s.command),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let todo = Todo::new();
        assert_eq!(todo.tasks.len(), 0);
    }

    #[test]
    fn test_add_todo() {
        let mut todo = Todo::new();
        todo.add_todo("Test task".to_string());
        assert_eq!(todo.tasks.len(), 1);
        assert_eq!(todo.tasks[0].name, "Test task");
        assert!(!todo.tasks[0].completed);
    }

    #[test]
    fn test_add_duplicate_todo() {
        let mut todo = Todo::new();
        todo.add_todo("Test task".to_string());
        todo.add_todo("Test task".to_string());
        assert_eq!(todo.tasks.len(), 1); // Should not add duplicate
    }

    #[test]
    fn test_get_status() {
        let mut todo = Todo::new();
        todo.add_todo("Test task".to_string());
        assert_eq!(todo.get_status("Test task"), Some(false));
        assert_eq!(todo.get_status("Non-existent"), None);
    }

    #[test]
    fn test_toggle() {
        let mut todo = Todo::new();
        todo.add_todo("Test task".to_string());
        todo.toggle("Test task");
        assert!(todo.tasks[0].completed);
        todo.toggle("Test task");
        assert!(!todo.tasks[0].completed);
    }

    #[test]
    fn test_delete() {
        let mut todo = Todo::new();
        todo.add_todo("Test task 1".to_string());
        todo.add_todo("Test task 2".to_string());
        assert_eq!(todo.tasks.len(), 2);
        todo.delete("Test task 1");
        assert_eq!(todo.tasks.len(), 1);
        assert_eq!(todo.tasks[0].name, "Test task 2");
    }

    #[test]
    fn test_clear() {
        let mut todo = Todo::new();
        todo.add_todo("Task 1".to_string());
        todo.add_todo("Task 2".to_string());
        todo.add_todo("Task 3".to_string());
        todo.toggle("Task 1");
        todo.toggle("Task 3");
        todo.clear();
        assert_eq!(todo.tasks.len(), 1);
        assert_eq!(todo.tasks[0].name, "Task 2");
    }

    #[test]
    fn test_reset() {
        let mut todo = Todo::new();
        todo.add_todo("Task 1".to_string());
        todo.add_todo("Task 2".to_string());
        assert_eq!(todo.tasks.len(), 2);
        todo.reset();
        assert_eq!(todo.tasks.len(), 0);
    }
}
