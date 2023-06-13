use clap::{ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File};
use std::io::{self, stdout, ErrorKind, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum TaskImportance {
    HIGH,
    MID,
    LOW,
}

#[derive(Serialize, Deserialize, Clone)]
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
        .subcommand(Command::new("read").about("Read current tasks in the db"))
        .get_matches()
}

fn main() -> std::result::Result<(), std::io::Error> {
    const DB_PATH: &str = "todo/db.json";

    let matches = cli();

    let mut task = Task {
        description: "This is my task".to_string(),
        completed: false,
        importance: TaskImportance::HIGH,
    };

    let mut task_list: Vec<Task> = Vec::new();

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

    match matches.subcommand() {
        Some(("add", _add)) => {
            println!("Add a task description.");

            print!("Description: ");
            stdout().flush()?;

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
                stdout().flush()?;

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

            task.description = description.clone();

            if fs::metadata(DB_PATH)?.len() != 0 {
                task_list = serde_json::from_str(&data)?;
            }

            task_list.push(task.clone());

            // Convert the data to a JSON string
            let json: String = serde_json::to_string_pretty(&task_list)?;

            fs::write(DB_PATH, &json).expect("Unable to write to db.");

            println!(
                "Adding task with description: '{description}' and importance {:?}",
                task.importance
            );
        }

        Some(("complete", _complete)) => {
            println!("Task complete")
        }
        Some(("read", _read)) => {
            let db = fs::read_to_string(DB_PATH).expect("Unable to reach db.");
            // let tasks = serde_json::from_str(&db);
            if db.is_empty() {
                println!("Your todo list is empty. Great job partner.");
            } else {
                // let obj = serde_json::from_str(&db)?;
                println!("{:#?}", db);
            };
        }
        _ => unreachable!(),
    }

    // println!("{}", &json);

    Ok(())
}
