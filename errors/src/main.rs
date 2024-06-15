use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let file_name = "src/main.rs";
    let content = File::open(file_name);
    let mut file = match content {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found! Trying to create one");
                match File::create(file_name) {
                    Ok(f) => f,
                    Err(err) => panic!("Failed to create a file {file_name}: {err:?}"),
                }
            }
            other => panic!("Not recoverable error: {other:?}")
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{content}");
}
