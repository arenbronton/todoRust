/*
TODO: --
& DISPLAY OPTIONS
& HANDLE CHOICE INPUT
& I/O FILE WRITING
*/

mod todo;
mod util;

use std::io::{self, Write};
use std::process::Command;
use util::colors::*;

fn main() {
    let mut todo_manager: todo::TodoManager = todo::TodoManager::new();
    todo_manager.download_tasks();

    clear_terminal();
    display_tasks(&todo_manager);

    let mut running: bool = true;
    while running {
        util::display_options();

        let mut input = String::new();
        print!("{}\n$ {}", GREEN, RESET);
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "clear" {
            clear_terminal();
            continue;
        }

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}Please enter a valid number.{}\n", RED, RESET);
                continue;
            }
        };

        match choice {
            1 => todo_manager.start_task_completion(),
            2 => todo_manager.start_task_creation(),
            3 => todo_manager.start_task_edit(),
            4 => todo_manager.start_task_deletion(),
            5 => display_tasks(&todo_manager),
            6 => {
                running = false;
                todo_manager.save_tasks();
                clear_terminal();
            }
            _ => println!("{}Invalid Choice{}\n", RED, RESET),
        }
    }
}

fn display_tasks(todo_manager: &todo::TodoManager) {
    println!("\n{} -= CURRENT TASKS: =- {}\n", RED, RESET);
    for task in todo_manager.get_tasks() {
        println!(
            "{}Name: {}\nDescription: {}\nID: {}\nCompleted: {}{}\n",
            BLUE, task.name, task.description, task.id, task.completed, RESET
        );
    }
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
