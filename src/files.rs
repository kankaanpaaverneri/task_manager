use crate::run::Task;
use std::{
    fs,
    io::{Error, Write},
};

const FILEPATH: &str = "tasks.txt";

pub fn create_new_file() -> Result<(), Error> {
    fs::File::create(FILEPATH)?;
    Ok(())
}

pub fn read_file_contents() -> Result<Vec<Task>, Error> {
    let result = fs::read_to_string(FILEPATH);

    match result {
        Ok(buffer) => {
            return Ok(parse_buffer(buffer));
        }
        Err(error) => Err(error),
    }
}

pub fn write_tasks_to_file(tasks: &Vec<Task>) -> Result<(), Error> {
    let mut buffer = String::new();
    read_tasks_to_buffer(tasks, &mut buffer);
    fs::write(FILEPATH, buffer)?;
    Ok(())
}

fn read_tasks_to_buffer(tasks: &Vec<Task>, buffer: &mut String) {
    for task in tasks {
        let formated_str = format!("{}:{}\n", task.task.as_str(), task.completed);
        buffer.push_str(formated_str.as_str());
    }
}

pub fn write_new_task_in_file(task: &Task) -> Result<usize, Error> {
    let file = fs::OpenOptions::new().append(true).open(FILEPATH);
    match file {
        Ok(mut f) => {
            let task_name = task.task.as_str();
            let completed = task.completed;
            let formated_task = format!("{task_name}:{completed}\n");
            f.write(formated_task.as_bytes())
        }
        Err(e) => Err(e),
    }
}

fn parse_buffer(buffer: String) -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();
    for line in buffer.lines() {
        let list: Vec<&str> = line.split(":").collect();
        if list.len() == 2 {
            let completed = parse_completed(list[1]);
            tasks.push(Task {
                task: String::from(list[0]),
                completed,
            });
        }
    }
    return tasks;
}

fn parse_completed(item: &str) -> bool {
    let mut completed = false;
    if item == "false" {
        completed = false;
    } else if item == "true" {
        completed = true;
    }

    return completed;
}
