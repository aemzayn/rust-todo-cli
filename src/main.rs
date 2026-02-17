use std::io::{Write, stdin, stdout};

struct Task {
    name: String,
    completed: bool,
}

struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn add_todo(&mut self, name: String) {
        self.tasks.push(Task {
            name: name.clone(),
            completed: false,
        });
        println!("Created a new task with name: {}", name)
    }

    #[allow(dead_code)]
    fn get_status(&mut self, name: String) -> Option<bool> {
        for task in &self.tasks {
            let task_name = task.name.clone();
            if task_name == name {
                return Some(task.completed);
            }
        }
        None
    }

    fn print_list(&self) {
        let total_tasks = self.tasks.len();
        println!("Found {} tasks", total_tasks);
        for task in &self.tasks {
            println!("{} status: {}", &task.name, &task.completed)
        }
    }

    fn delete(&mut self, name: String) {
        println!("Deleted {}", name);
    }

    fn toggle(&mut self, name: String) {
        println!("Toggled {}", name);
    }

    fn clear(&mut self) {
        println!("All completed tasks has been cleared!");
    }

    fn reset(&mut self) {
        println!("All tasks has been deleted");
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
    match s.find(" ") {
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

        if s.command == "new" {
            if s.arg == "" {
                println!("Missing todo name!")
            } else {
                todo.add_todo(s.arg);
            }
        } else if s.command == "list" {
            todo.print_list();
        } else if s.command == "done" {
            todo.toggle(s.arg);
        } else if s.command == "delete" {
            todo.delete(s.arg);
        } else if s.command == "reset" {
            todo.reset();
        } else if s.command == "clear" {
            todo.clear();
        } else if s.command == "exit" {
            println!("Terminating the Rust TODO");
            break;
        }
    }
}
