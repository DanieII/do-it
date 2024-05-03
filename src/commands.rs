use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader};

fn get_todos() -> io::Result<Vec<String>> {
    let file_path: String = super::file::get_file_path();
    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("Could not open file");
    let reader = BufReader::new(file);

    reader.lines().collect()
}

pub fn list_todos() {
    let todos = get_todos().expect("Could not read todos");

    for (i, todo) in todos.iter().enumerate() {
        let number = i + 1;

        println!("{} {}", number, todo);
        if number < todos.len() {
            println!()
        }
    }
}

pub fn add_todo(args: &Vec<String>) {
    if args.len() == 2 {
        println!("A text for the todo must be provided as a second argument");
        return;
    }

    let todo = args[2].to_string();
    let file_path: String = super::file::get_file_path();
    let file = OpenOptions::new()
        .append(true)
        .open(&file_path)
        .expect("Could not open file");

    super::file::append_to_file(&todo, &file)
}

pub fn remove_todo(args: &Vec<String>) {
    if args.len() == 2 {
        println!("You must provide at least one todo number");
        return;
    }
    for arg in &args[2..] {
        if !super::utils::is_string_numeric(arg) {
            println!("Make sure to provide numbers for the todos");
            return;
        }
    }

    let todos: Vec<String> = get_todos().expect("Could not read todos");
    let numbers_to_delete: Vec<i32> = args[2..]
        .iter()
        .map(|n| n.parse::<i32>().expect("Could not get number"))
        .collect();
    let file_path = super::file::get_file_path();
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Could not open file");

    for (i, todo) in todos.iter().enumerate() {
        let todo_number = (i + 1) as i32;

        if !numbers_to_delete.contains(&todo_number) {
            super::file::append_to_file(todo, &file);
        }
    }
}
