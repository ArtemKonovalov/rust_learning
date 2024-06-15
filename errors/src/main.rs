use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let file_name = "src/main.rs";
    let open_file_result = File::open(file_name);
    let mut file = match open_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found! Trying to create one");
                match File::create(file_name) {
                    Ok(f) => f,
                    Err(err) => panic!("Failed to create a file {file_name}: {err:?}"),
                }
            }
            other_error => panic!("Not recoverable error: {other_error:?}")
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{content}");
}
