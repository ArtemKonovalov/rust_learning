enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn demo_vector() {
    let v = vec![1, 2, 3];
    println!("Vector length: {}", v.len());

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Vector length: {}", v.len());

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is: {}", third);

    // I guess it's not breaking because of primitives we use there that may be Copied
    let mut third = v[2];
    third = 66;
    for i in &v {
        println!("Item {}", i)
    }
    println!("The third element is: {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        None => println!("There is no third element"),
        Some(th) => println!("The third element is {th}"),
    }

    let mut v = vec![199, 234, 12];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("Updated item: {}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Text(String::from("green")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Float(11.22),
    ];

    for r in &row {
        match r {
            SpreadsheetCell::Int(i) => println!("integer {}", i),
            SpreadsheetCell::Float(f) => println!("float {}", f),
            SpreadsheetCell::Text(s) => println!("string {}", s)
        }
    }
}