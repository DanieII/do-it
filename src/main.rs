use std::io;

fn main() {
    // let mut tasks: Vec<String> = Vec::new();
    let mut tasks: Vec<String> = vec!["test1".to_string(), "test2".to_string()];
    let mut is_active: bool = true;

    while is_active {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => show_tasks(&mut tasks),
            _ => break,
        }
    }
}

fn add_task(tasks: &mut Vec<String>) {
    let mut task_name = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("Failed to read task name");

    tasks.push(task_name.clone());
    println!("{}", task_name);
}

fn show_tasks(tasks: &mut Vec<String>) {
    for task in tasks {
        println!("{}", task)
    }
}
