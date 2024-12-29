use crate::args::Config;
use crate::files;
use std::io::{Error, ErrorKind};
use std::usize;

pub struct Task {
    pub task: String,
    pub completed: bool,
}

pub fn run(config: &Config) {
    //display_config(config);
    match files::read_file_contents() {
        Ok(list) => {
            run_action(config, &list);
        }
        Err(e) => {
            if let Ok(()) = handle_file_error(e) {
                let list = Vec::new();
                run_action(config, &list);
                return;
            }
        }
    }
}

fn handle_file_error(error: Error) -> Result<(), std::io::Error> {
    if error.kind() == ErrorKind::NotFound {
        eprintln!("File not found. Creating new file");
        return match files::create_new_file() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
    eprintln!("{}", error);
    return Err(error);
}

fn run_action(config: &Config, list: &Vec<Task>) {
    if config.first == "list" {
        display_list(list);
    }

    if config.first == "add" {
        add_to_list(&config.second);
    }

    if config.first == "complete" {
        mark_as_complete(&config.second);
    }

    if config.first == "remove" {
        remove_task(&config.second);
    }
}

fn add_to_list(task_name: &String) {
    // Write in to a file
    let corrected_task_name = remove_invalid_characters(task_name);
    let task = Task {
        task: corrected_task_name,
        completed: false,
    };
    match files::write_new_task_in_file(&task) {
        Err(e) => eprintln!("{}", e),
        Ok(_) => {
            println!("Write to file success")
        }
    }
}

struct FileContent {
    tasks: Vec<Task>,
    line_number: Option<usize>,
}

fn read_file_tasks(second_argument: &String) -> Result<FileContent, Error> {
    let line_number = validate_second_argument(second_argument);
    let result = files::read_file_contents();
    match result {
        Ok(tasks) => {
            return Ok(FileContent { tasks, line_number });
        }
        Err(e) => Err(e),
    }
}

fn mark_as_complete(second_argument: &String) {
    let file_content = read_file_tasks(second_argument);
    match file_content {
        Ok(mut content) => {
            let line_number: usize = content.line_number.unwrap_or_else(|| {
                println!("Line number not valid");
                std::process::exit(1);
            });
            modify_complete_status(&mut content.tasks, line_number);
            if let Err(e) = files::write_tasks_to_file(&content.tasks) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn remove_task(second_argument: &String) {
    let file_content = read_file_tasks(second_argument);
    match file_content {
        Ok(mut content) => {
            let line_number = content.line_number.unwrap_or_else(|| {
                println!("Line number not valid");
                std::process::exit(1);
            });
            content.tasks = remove_task_by_line(&mut content.tasks, line_number);
            if let Err(e) = files::write_tasks_to_file(&content.tasks) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn remove_task_by_line(tasks: &mut Vec<Task>, line_number: usize) -> Vec<Task> {
    let mut filtered_task: Vec<Task> = Vec::new();
    for (index, task) in tasks.iter().enumerate() {
        if index + 1 != line_number {
            filtered_task.push(Task {
                task: task.task.clone(),
                completed: task.completed,
            });
        }
    }
    return filtered_task;
}

fn validate_second_argument(second_argument: &String) -> Option<usize> {
    if second_argument.len() < 1 {
        eprintln!("Second argument not provided");
        return None;
    }
    return Some(second_argument.parse().unwrap_or_else(|e| -> usize {
        eprintln!("{}", e);
        std::process::exit(1);
    }));
}

fn modify_complete_status(tasks: &mut Vec<Task>, line_number: usize) {
    if line_number <= 0 || line_number > tasks.len() {
        println!("Line number is not valid");
    }
    for (index, task) in tasks.iter_mut().enumerate() {
        if index + 1 == line_number {
            task.completed = true;
        }
    }
}

fn remove_invalid_characters(task_name: &String) -> String {
    let mut corrected_task_name = String::new();
    for character in task_name.chars() {
        if character != ':' {
            corrected_task_name.push(character);
        }
    }
    return corrected_task_name;
}

fn display_list(list: &Vec<Task>) {
    println!("TASKS\n\n");
    if list.len() == 0 {
        println!("List is empty");
        return;
    }
    for (index, task) in list.iter().enumerate() {
        println!(
            "{}. task: {} | completed: {}",
            index + 1,
            task.task,
            task.completed
        );
    }
}
