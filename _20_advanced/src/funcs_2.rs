fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // gets function pointer
    let arg = f(arg);
    f(arg)
}

fn do_twice2<T, U>(f: T, arg: U) -> U
where
    T: Fn(U) -> U,
{
    // gets function pointer
    let arg = f(arg);
    f(arg)
}

// Return closure
fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn _returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect(); // passing closure

    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}
