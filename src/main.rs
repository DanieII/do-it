mod commands;
mod file;
mod utils;
use std::env;

fn main() {
    file::create_file();
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let command = &args[1];
        match command.as_str() {
            "add" => commands::add_todo(&args),
            "ls" => commands::list_todos(),
            "rm" => commands::remove_todo(&args),
            _ => println!("{} is not an available command", command),
        }
    } else {
        println!("doit is a cli written in rust")
    }
}
