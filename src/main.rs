use std::io::{Write, stdin, stdout};

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

        // for task in &self.tasks {
        //     if task.name == name {
        //         print!("Task with name {} already exists!", name);
        //         return;
        //     }
        // }

        if self.tasks.iter().any(|task| task.name == name) {
            println!("Task with name {} already exists!", name);
            return;
        }

        let task_name = name.clone();
        self.tasks.push(Task {
            name,
            completed: false,
        });
        println!("Created a new task with name: {}", task_name);
    }

    #[allow(dead_code)]
    fn get_status(&mut self, name: String) -> Option<bool> {
        // for task in &self.tasks {
        //     let task_name = task.name.clone();
        //     if task_name == name {
        //         return Some(task.completed);
        //     }
        // }
        // None
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
        // self.tasks = self
        //     .tasks
        //     .iter()
        //     .filter(|&task| task.name != name)
        //     .cloned()
        //     .collect();

        self.tasks.retain(|task| task.name != name);

        let total_after = self.tasks.len();
        if total_after < total_before {
            println!("Deleted '{}'", name);
        } else {
            println!("Cannot find task '{}'", name);
        }
    }

    fn toggle(&mut self, name: &str) {
        // for task in &mut self.tasks {
        //     if task.name == name {
        //         task.completed = !task.completed;
        //         println!("Toggled {}", name);
        //         return;
        //     }
        // }
        if let Some(task) = self.tasks.iter_mut().find(|task| task.name == name) {
            task.completed = !task.completed;
            let status = if task.completed {
                "completed"
            } else {
                "incomplete"
            };
            println!("Marked '{}' as {}", name, status)
        } else {
            println!("Cannot find task '{}'", name);
        }
    }

    fn clear(&mut self) {
        let total_before = self.tasks.len();
        // self.tasks = self
        //     .tasks
        //     .iter()
        //     .filter(|&task| task.completed != true)
        //     .cloned()
        //     .collect();

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
- help: Show available commands
- exit: Exit the app
    "#
    );

    loop {
        print!("# ");
        let s = get_input();

        match s.command.as_str() {
            "new" => {
                if s.arg.is_empty() {
                    println!("Missing task name");
                } else {
                    todo.add_todo(s.arg);
                }
            }
            "list" => todo.print_list(),
            "done" => todo.toggle(&s.arg),
            "remove" | "delete" => todo.delete(&s.arg),
            "reset" => todo.reset(),
            "clear" => todo.clear(),
            "help" => {
                println!(
                    r#"
Available commands:
- new <name>: Create new task
- list: See all current tasks
- done <name>: Mark a task as completed
- clear: Clear all completed tasks
- reset: Clear all tasks
- remove <name>: Remove a task
- help: Show this help message
- exit: Exit the app
                    "#
                );
            }
            "exit" => {
                println!("Terminating Rust TODO");
                break;
            }
            "" => {} // ignore empty command
            _ => println!(
                "Unknown commmand: '{}'. Type 'help' to see available commands.",
                s.command
            ),
        }
    }
}
