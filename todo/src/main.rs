use clap::{ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File};
use std::io::{self, stdout, ErrorKind, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
enum TaskImportance {
    HIGH,
    MID,
    LOW,
}

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
    importance: TaskImportance,
}

fn cli() -> ArgMatches {
    Command::new("todo")
        .subcommand_required(true)
        .subcommand(Command::new("add").about("Add to todo list"))
        .subcommand(Command::new("complete").about("Mark task as complete"))
        .get_matches()
}

fn main() -> std::io::Result<()> {
    const DB_PATH: &str = "todo/db.json";

    let matches = cli();

    let mut task = Task {
        description: "This is my task".to_string(),
        completed: false,
        importance: TaskImportance::HIGH,
    };

    match matches.subcommand() {
        Some(("add", add)) => {
            println!("Add a task description.");

            print!("Description: ");
            stdout().flush().unwrap();

            let mut description = String::new();

            io::stdin()
                .read_line(&mut description)
                .expect("Failed to record description.");

            let description: String = match description.trim().parse() {
                Ok(desc) => desc,
                Err(err) => panic!("Issue reading description {:?}", err),
            };

            loop {
                println!("What is the importance of the task? (HIGH, MID, LOW)");

                print!("Importance: ");
                stdout().flush().unwrap();

                let mut importance = String::new();

                io::stdin()
                    .read_line(&mut importance)
                    .expect("Failed to record importance.");

                let importance: String = match importance.trim().parse() {
                    Ok(i) => i,
                    Err(_) => continue,
                };

                let importance = match importance.to_uppercase().as_str() {
                    "HIGH" => TaskImportance::HIGH,
                    "MID" => TaskImportance::MID,
                    "LOW" => TaskImportance::LOW,
                    _ => {
                        println!("{importance} was not an option. Choose from HIGH, MID, or LOW");
                        continue;
                    }
                };

                task.importance = importance;
                break;
            }

            println!(
                "Adding task with description: '{description}' and importance {:?}",
                task.importance
            );
            task.description = description;
        }

        Some(("complete", complete)) => {
            println!("Task complete")
        }
        _ => unreachable!(),
    }

    let f = File::open(DB_PATH);

    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(DB_PATH) {
                Ok(new_file) => new_file,
                Err(e) => panic!("Problem creating the file {:?}.", e),
            },
            _ => todo!(),
        },
    };

    let data = fs::read_to_string(DB_PATH).expect("Unable to reach db.");

    let mut task_list: Vec<Task> = Vec::new();

    if fs::metadata(DB_PATH).unwrap().len() != 0 {
        task_list = serde_json::from_str(&data)?;
    }

    task_list.push(task);

    // Convert the data to a JSON string
    let json: String = serde_json::to_string_pretty(&task_list)?;

    fs::write(DB_PATH, &json).expect("Unable to write to db.");

    // println!("{}", &json);

    Ok(())
}
