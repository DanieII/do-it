use colored::Colorize;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    create_file();
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let action = &args[1];
        match action.as_str() {
            "add" => add_todo(&args),
            "ls" => list_todos(),
            "rm" => remove_todo(&args),
            _ => println!("{} is not an available action", action),
        }
    } else {
        println!("doit is a cli written in rust")
    }
}

fn get_file_path() -> String {
    let file_path = match env::var("HOME") {
        Ok(path) => path + "/.doit",
        Err(_) => ".doit".to_string(),
    };

    file_path
}

fn create_file() {
    let file_path: String = get_file_path();
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_path);

    match file {
        Ok(_) => return,
        Err(e) => {
            if e.kind() != io::ErrorKind::AlreadyExists {
                panic!("Could not create file")
            }
        }
    }
}

fn get_todos() -> io::Result<Vec<String>> {
    let file_path: String = get_file_path();
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    reader.lines().collect()
}

fn list_todos() {
    let todos = get_todos().expect("Could not read todos");

    for (i, todo) in todos.iter().enumerate() {
        let number = i + 1;

        println!("{} {}", number, todo);
        if number < todos.len() {
            println!()
        }
    }
}

fn add_todo(args: &Vec<String>) {
    if args.len() == 2 {
        println!("A text for the todo must be provided as a second argument");
        return;
    }

    let todo = args[2].to_string();
    let file_path: String = get_file_path();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .expect("Could not open file");

    let text = todo.trim().to_string() + "\n";
    file.write_all(text.as_bytes())
        .expect("Could not append to file");
}

fn remove_todo(args: &Vec<String>) {
    if args.len() == 2 {
        println!("A todo number must be provided as a second argument");
        return;
    }
    if !is_string_numeric(&args[2]) {
        println!("The second argument must be a number")
    }
}

fn is_string_numeric(str: &String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
