use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

// Way for rust to return something or error
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

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

// Does the same
// operator ? in rust moves value to variable if Ok
// if Err propagates it
// It can only be used if function return type is compatible with fn return type
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99];

    // Handling functions that return Result<T, E>
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {error:?}");
    });

    // The same, calls panic! auto generated text
    let greeting_file = File::open("hello.txt").unwrap();
    // The same, lets us specify panic! text
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
