
pub fn demo_strings() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    // this also works
    let s = "initial contents".to_string();
    // that's the same
    let mut s = String::from("initial contents");

    // Appending
    s.push_str(", bar");

    // Using string slice without taking ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('!');

    println!("s2 is {s2}; s1 is {s1}");

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been MOVED - can no longer be used
    // the '+' operator uses add method with the following signature
    // fn add(self, s: &str) -> String {}
    // &s2 is actually &String, but the compiler can COERCE the &String argument into a &str
    // Rust uses a deref coercion, which here turns &s2 into &s2[..]

    println!("S3: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("concat of 3 (1): {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! macro doesn't take ownership of any parameters
    let s = format!("{s1}-{s2}-{s3}");
    println!("concat of 3 (2): {s}");
}