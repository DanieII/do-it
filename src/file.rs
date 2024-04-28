use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};

pub fn get_file_path() -> String {
    let file_path = match env::var("HOME") {
        Ok(path) => path + "/.doit",
        Err(_) => ".doit".to_string(),
    };

    file_path
}

pub fn create_file() {
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

pub fn append_to_file(todo: &String, mut file: &File) {
    let text = todo.trim().to_string() + "\n";
    file.write_all(text.as_bytes())
        .expect("Could not append to file");
}
