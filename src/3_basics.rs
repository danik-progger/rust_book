fn main() {
    // --- will cause error
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // --- will work as expected
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    // The value of x in the inner scope is: 12
    // The value of x is: 6

    // --- tuples
    let tup: (i32, f32, &str) = (1, 3.14, "hey");
    let (x, y, z) = tup;
    let x1 = tup.0;
    let x2 = tup.1;
    let x3 = tup.2;

    // --- arr
    let arr = [1, 2, 3, 4];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let a1 = a[0];
    let a2 = a[1];
    let a3 = a[2];

    // --- statement expression
    let y = {
        let x = 5;
        x + 1
    };

    // --- conditions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = if number == 3 { 5 } else { 6 };

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returns value from loop
        }
    };

    let mut count = 0;
    'counting_up: loop {
        // give loop a name
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // break loop with this name
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}

fn another_function(x: i32) -> i32 {
    println!("Passed parametr is {x}");
    x * 2
}
