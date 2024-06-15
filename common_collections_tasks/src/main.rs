use vector_analyzer::median;
use vector_analyzer::mode;

mod vector_analyzer;
mod string_convertor;
mod text_interface;

fn main() {
    let v = vec![1, 2, 3, 3, 3, 2, 2, 4, 5, 5, 5, 5, 5, 5];

    let mode = mode(&v);
    let median = median(&v);

    println!("Mode:   {mode}");
    println!("Median: {median}");
}

