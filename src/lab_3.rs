use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Todo {
    id: usize,
    description: String,
    done: bool,
}

struct TodoList {
    tasks: Vec<Todo>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Todo {
            id: self.next_id,
            description,
            done: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("Task added successfully.");
    }

    fn remove_task(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            println!("Task removed successfully.");
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn edit_task(&mut self, id: usize, new_description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.description = new_description;
            println!("Task updated successfully.");
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn mark_done(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            println!("Task marked as done.");
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            println!("\nTodo List:");
            for task in &self.tasks {
                println!("[{}] {} - {}", task.id, task.description, if task.done { "Done" } else { "Pending" });
            }
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
            .expect("Unable to open file.");

        for task in &self.tasks {
            writeln!(
                file,
                "{}, {}, {}",
                task.id, task.description, task.done
            )
                .expect("Unable to write to file.");
        }
        println!("Tasks saved to file.");
    }

    fn load_from_file(&mut self, filename: &str) {
        self.tasks.clear();
        if let Ok(contents) = fs::read_to_string(filename) {
            for line in contents.lines() {
                let parts: Vec<&str> = line.splitn(3, ',').collect();
                if parts.len() == 3 {
                    if let (Ok(id), Ok(done)) = (
                        parts[0].trim().parse::<usize>(),
                        parts[2].trim().parse::<bool>(),
                    ) {
                        self.tasks.push(Todo {
                            id,
                            description: parts[1].trim().to_string(),
                            done,
                        });
                        self.next_id = self.next_id.max(id + 1);
                    }
                }
            }
            println!("Tasks loaded from file.");
        } else {
            println!("No saved tasks found.");
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let filename = "tasks.txt";
    todo_list.load_from_file(filename);

    loop {
        println!("\n+-----------------------------+");
        println!("| Todo List Application       |");
        println!("+-----------------------------+");
        println!("| 1. Add Task                 |");
        println!("| 2. Remove Task              |");
        println!("| 3. Edit Task                |");
        println!("| 4. Mark Task as Done        |");
        println!("| 5. List Tasks               |");
        println!("| 6. Save and Exit            |");
        println!("+-----------------------------+");

        let choice = TodoList::get_input("Choose an option:");

        match choice.as_str() {
            "1" => {
                let description = TodoList::get_input("Enter task description:");
                todo_list.add_task(description);
            }
            "2" => {
                let id = TodoList::get_input("Enter task ID to remove:");
                match id.parse::<usize>() {
                    Ok(id) => todo_list.remove_task(id),
                    Err(_) => println!("Invalid ID."),
                }
            }
            "3" => {
                let id = TodoList::get_input("Enter task ID to edit:");
                match id.parse::<usize>() {
                    Ok(id) => {
                        let description = TodoList::get_input("Enter new description:");
                        todo_list.edit_task(id, description);
                    }
                    Err(_) => println!("Invalid ID."),
                }
            }
            "4" => {
                let id = TodoList::get_input("Enter task ID to mark as done:");
                match id.parse::<usize>() {
                    Ok(id) => todo_list.mark_done(id),
                    Err(_) => println!("Invalid ID."),
                }
            }
            "5" => todo_list.list_tasks(),
            "6" => {
                todo_list.save_to_file(filename);
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

impl TodoList {
    fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }
}
