// Using a hash map and vectors,
// create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::{
    collections::HashMap,
    io,
};

pub fn start() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    println!("Hello there! Let's start our employees audit.
Here is an instruction of how to use this \"app\"
First, we need to add an employee to a specific department. Command example:
`Add Mike to Developers`
This will add employee Mike to Developers department.");
    // TODO: Finish README
    // TODO: Implement case insensitivity

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");

        let words: Vec<String> = command.split_whitespace()
            .map(str::to_string)
            .collect();

        match parse_command(words) {
            Command::Add { who, to_where } => {
                departments.entry(to_where).or_insert(Vec::new()).push(who);
            }
            Command::Remove { who, from_where } => {
                let employees_opt = departments.get_mut(from_where.as_str());
                if let Some(list) = employees_opt {
                    if let Some(index) = list.iter().position(|x| *x == who) {
                        list.remove(index);
                    }
                }
            }
            Command::List { department } => {
                match department {
                    None => {
                        println!("{:?}", departments);
                    }
                    Some(department_name) => {
                        println!("{:?}", departments.get(department_name.as_str()));
                    }
                }
            },
            Command::Exit => {
                break;
            }
        }
    }
}

fn parse_command(words: Vec<String>) -> Command {
    let first_word = words.get(0).expect("No words...");
    match first_word.as_str() {
        "Add" => {
            if words.len() != 4 || "to".eq(words.get(3).unwrap()) {
                panic!("Incorrect format of command. Expected [Add <employee_name> to <department_name>]");
            }
            Command::Add {
                who: get_word_by_index(&words, 1),
                to_where: get_word_by_index(&words, 3),
            }
        }
        "Remove" => Command::Remove {
            who: get_word_by_index(&words, 1),
            from_where: get_word_by_index(&words, 3),
        },
        "Delete" => Command::Remove {
            who: get_word_by_index(&words, 1),
            from_where: get_word_by_index(&words, 3),
        },
        "List" => {
            Command::List {
                department: match words.get(1) {
                    Some(value) => Some(value.clone()),
                    None => None,
                },
            }
        },
        "Exit" => Command::Exit,
        _ => Command::List {
            department: None,
        },
    }
}

fn get_word_by_index(vec: &Vec<String>, index: usize) -> String {
    vec.get(index).unwrap().to_string()
}

enum Command {
    Add {
        who: String,
        to_where: String,
    },
    Remove {
        who: String,
        from_where: String,
    },
    List {
        department: Option<String>,
    },
    Exit,
}