use crate::util;
use crate::util::colors::*;

use rand::Rng;
use std::fs;

pub struct Task {
    pub name: String,
    pub id: u8,
    pub description: String,
    pub completed: bool,
}

impl Task {
    #[allow(unused)]
    pub fn new(name: &str, id: u8, desc: &str) -> Self {
        Task {
            name: name.to_string(),
            id,
            description: desc.to_string(),
            completed: false,
        }
    }
}

pub struct TodoManager {
    tasks: Vec<Task>,
}

impl TodoManager {
    pub fn new() -> Self {
        TodoManager { tasks: Vec::new() }
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn create_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn start_task_completion(&mut self) {
        let task_id_str: String = util::input("Enter task ID: ");
        let task_id: u8 = match task_id_str.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid ID. Please enter a valid number.");
                return;
            }
        };

        if let Some(task) = self.tasks.iter_mut().find(|t: &&mut Task| t.id == task_id) {
            // Modify the task
            task.completed = true;
            println!(
                "Task marked as completed:\nName: {}, Description: {}",
                task.name, task.description
            );
        } else {
            println!("No task found with ID: {}", task_id);
        }
    }

    pub fn start_task_creation(&mut self) {
        let task_name: String = util::input("Enter task name: ");
        let task_desc: String = util::input("Enter task description: ");
        let task_id: u8 = self.generate_id();
        let task: Task = Task {
            name: task_name,
            id: task_id,
            description: task_desc,
            completed: false,
        };
        self.tasks.push(task);
        println!("{}Task created!{}\n", GREEN, RESET);
    }

    pub fn start_task_edit(&mut self) {
        let task_id_str: String = util::input("Enter task ID: ");
        let task_id: u8 = match task_id_str.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid ID. Please enter a valid number.");
                return;
            }
        };

        if let Some(task) = self.tasks.iter_mut().find(|t: &&mut Task| t.id == task_id) {
            // Modify the task
            println!("Which field are you editing?\n1. Name\n2. Description\n");
            let field: String = util::input(&format!("{}${} ", GREEN, RESET));

            if field == "1".to_string() {
                let new_name: String = util::input("Enter new name: ");
                task.name = new_name;
                println!("Updated Task Name: {}", task.name);
            } else if field == "2".to_string() {
                let new_desc: String = util::input("Enter new description: ");
                task.description = new_desc;
                println!("Updated Task Description: {}", task.description);
            } else {
                println!("Invalid choice")
            }
        } else {
            println!("No task found with ID: {}", task_id);
        }
    }

    pub fn start_task_deletion(&mut self) {
        let task_id_str: String = util::input("Enter task ID: ");
        let task_id: u8 = match task_id_str.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid ID. Please enter a valid number.");
                return;
            }
        };

        if let Some(task) = self.tasks.iter_mut().find(|t: &&mut Task| t.id == task_id) {
            // Modify the task
            println!("Task Deleted: {}", task.name);
        } else {
            println!("No task found with ID: {}", task_id);
        }

        self.tasks.retain(|t: &Task| t.id != task_id);
    }

    pub fn download_tasks(&mut self) {
        // IO logic
        let data: String = fs::read_to_string("./tasks.txt").expect("Failed to read data file");
        // Organize data
        let lines: Vec<&str> = data.split("\n").collect();
        for line in lines {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                if let Ok(id) = parts[2].trim().parse::<u8>() {
                    let new_task: Task = Task {
                        name: parts[0].to_string(),
                        id,
                        description: parts[1].to_string(),
                        completed: if parts[3].to_string() == "true" {
                            true
                        } else {
                            false
                        },
                    };
                    // Do something with new_task
                    self.create_task(new_task);
                } else {
                    println!("Failed to parse ID: {}", parts[2]);
                }
            } else {
                println!("Incorrect number of fields in line: {}", line);
            }
        }
    }

    pub fn save_tasks(&mut self) {
        let mut data: String = String::new();
        for (i, task) in self.tasks.iter().enumerate() {
            if i == self.tasks.len() - 1 {
                data.push_str(&format!(
                    "{},{},{},{}",
                    task.name, task.description, task.id, task.completed
                ));
            } else {
                data.push_str(&format!(
                    "{},{},{},{}\n",
                    task.name, task.description, task.id, task.completed
                ));
            }
        }
        // IO
        fs::write("./tasks.txt", data).expect("Failed to write data to file");
    }

    pub fn generate_id(&mut self) -> u8 {
        let mut rng = rand::thread_rng();
        return rng.gen::<u8>();
    }
}
