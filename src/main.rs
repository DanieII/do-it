mod commands;
mod file;
mod utils;
use std::env;

const HELP_MESSAGE: &str = "Doit is a simple cli program for managing tasks
Commands:
    - add [TASK]
        adds a new task
    - ls
        lists all tasks
    - rm [NUMBER/S]
        removes one or more tasks";

fn main() {
    file::create_file();
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let command = &args[1];
        match command.as_str() {
            "add" => commands::add_todo(&args),
            "ls" => commands::list_todos(),
            "rm" => commands::remove_todo(&args),
            "help" | "-h" | "--help" | _ => println!("{}", HELP_MESSAGE),
        }
    } else {
        println!("{}", HELP_MESSAGE)
    }
}
