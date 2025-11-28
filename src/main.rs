//serde means serilization and deserilization serilization means taking the data and converting to json and vice versa .
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)] //Debug is used to print the struct
struct Todo {
    //PascalCase
    pub title: String,
    pub description: String,
    pub completed: bool,
}

fn save_todos(todos: &Vec<Todo>) -> Result<(), Box<dyn Error>> {
    //means can returnn any thype of the errors.why box because to fix the size rust dont know how much size the error takes so we wrap in box

    let file = File::create("db.json")?;
    serde_json::to_writer(file, todos)?; //? operator is used to propagate the error if any
    Ok(())
}

fn load_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    let file = File::open("db.json");
    match file {
        Ok(file) => {
            let todos: Vec<Todo> = serde_json::from_reader(file)?; //from_reader is
            Ok(todos)
        }
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            Ok(Vec::new()) //if file not found return empty vector
        }
        Err(e) => {
            Err(Box::new(e)) //other errors propagate
        } // think what happen if first time file open no data no file so error ut that actually is that the error? no so we uses match to handel that properly
    }
}

fn main() {
    let mut todos: Vec<Todo> = load_todos().unwrap_or_else(|_| Vec::new()); //mutable vector (Vec) to hold instances
    println!("Welcome to the Todo App!");
    println!("welcome to todo cli type 'add' 'list' 'complete' or 'exit' ");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim();
        if command == "add" {
            let mut title = String::new();
            let mut description = String::new();
            println!("Enter the title of the task:");
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line");
            println!("Enter the description of the task:");
            io::stdin()
                .read_line(&mut description)
                .expect("Failed to read line");
            let todo = Todo {
                title: title.trim().to_string(),
                description: description.trim().to_string(),
                completed: false,
            };
            todos.push(todo);
            println!("Task added successfully!");
        } else if command == "list" {
            println!("Current Tasks:");
            for (index, todo) in todos.iter().enumerate() {
                //emurating means getting index and value both
                println!(
                    "{}. {} - {} [{}]",
                    index + 1,
                    todo.title,
                    todo.description,
                    if todo.completed {
                        "Completed"
                    } else {
                        "Pending"
                    }
                ); //no ternary operator in rust so we use if todo.completed{"Completed"}else{"Pending"} 
            }
        }  else if command == "complete" {
    println!("Enter the task number to mark as completed:");
    let mut task_number = String::new();
    
    io::stdin()
        .read_line(&mut task_number)
        .expect("Failed to read line");

    let task_index_option = match task_number.trim().parse::<usize>() {
        Ok(num) => {
            
            if num > 0 {
                Some(num - 1) 
            } else {
                None
            }
        },
        Err(_) => None,
    };
    match task_index_option {
        Some(index) => {
             if index < todos.len() {
                todos[index].completed = true;
                println!("Task marked as completed.");
            } else {
                println!("Task number out of range.");
            }
        }
        None => println!("Invalid task number."),
    }
}else if command == "exit" {
            break;
        } else {
            println!("Invalid command. Please type add, list,complete or exit.");
        }
    }
    
    println!("current task =>>>>{:#?}", todos); //printing the vector in a pretty format if not use # then not preety 
    match save_todos(&todos) {
        Ok(_) => println!("Todos saved successfully."),
        Err(e) => eprintln!("Failed to save todos: {}", e),
    }
}


//in c++ we use string s;
//cin>>s;
//but in rust we use let mut s=String::new();
//std::io::stdin().read_line(&mut s).expect("Failed to read line");
//&mut s means we are passing the reference of the string s to the read_line function
//expect is used to handle the error if any
//builtin  formatter to format the code use cargo fmt.
