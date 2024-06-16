use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

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

    /*
    .unwrap() returns the value from Ok, or panic! on Err;
    .expect() behaves the same but we can control an error message.
     */

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{content}");
}

fn alternatives_to_match_with_results() {
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short_version() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_orig() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/**
Operator ? can be used as with Result, as with Option for early return
*/
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
