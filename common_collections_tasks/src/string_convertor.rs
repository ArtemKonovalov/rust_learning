// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

pub fn pig_latin(str: &str) -> String {
    let first_letter = &str.chars().nth(0).unwrap();
    let other_letters = &str[1..];
    let mut res = String::new();
    let suffix;
    if is_vowel(first_letter) {
        res.push_str(str);
        suffix = "hay".to_string();
    } else {
        res.push_str(other_letters);
        suffix = format!("{first_letter}ay");
    }
    res.push('-');
    res.push_str(&suffix);

    res
}

fn is_vowel(char: &char) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    let low_char = char.to_lowercase().to_string().chars().nth(0).unwrap();
    vowels.contains(&low_char)
}