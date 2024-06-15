use vector_analyzer::mode;
use vector_analyzer::median;
use string_convertor::pig_latin;
use text_interface::start;
mod vector_analyzer;
mod string_convertor;
mod text_interface;

fn main() {
    let v = vec![1, 2, 3, 3, 3, 2, 2, 4, 5, 5, 5, 5, 5, 5];

    let mode = mode(&v);
    let median = median(&v);

    println!("Mode:   {mode}");
    println!("Median: {median}");

    modify_and_print_words(vec!["first", "apple"]);

    start();
}

fn modify_and_print_words(words: Vec<&str>) {
    for original_word in words {
        let modified_word = pig_latin(original_word);
        println!("Original word '{original_word}' is changed to '{modified_word}'");
    }
}
