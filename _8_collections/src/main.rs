use std::collections::HashMap;

fn vector() {
    println!("\nVector");

    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; -> panic
    // let does_not_exist = v.get(100); -> None

    // Error
    // let first = &v[0]; - immutable borrow
    // v.push(6); - mutable borrow
    // println!("The first element is: {first}"); - immutable borrow again

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    // Use enums to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn string() {
    println!("\nString");

    let mut s = String::new();

    let data = "initial contents";
    // Next 3 do the same
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let s = &s[0..4];

    let mut s = String::from("foo");
    s.push_str("bar");
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    for c in "Lol".chars() {
        println!("{c}");
    }
}

fn hashmap() {
    println!("\nHashmap");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    let field_name = String::from("Yellow");
    let field_value = 50;
    scores.insert(field_name, field_value);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    scores.entry(String::from("Yellow")).or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    vector();
    string();
    hashmap();
}
