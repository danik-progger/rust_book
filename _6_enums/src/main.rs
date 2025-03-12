enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8), // we can specify type for enum
    V6(String),
}

// The definition of option in standart library
// enum Option<T> {
//     None,
//     Some(T),
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
        _ => 2, // all other scenarios
    }
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    let four = IpAddrKind::V4;
    route(IpAddrKind::V6);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // --- if let
    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    // behaves the same
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
