

fn bar() -> ! { // never returns
    loop {}
}

// impl<T> _Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

// Sized = size is known at compile time
fn _generic<T: Sized>(_t: T) { // default
    // --snip--
}

fn _generic2<T: ?Sized>(_t: &T) { // we can overwrite it
    // --snip--
}

fn main() {
    type Kilometers = i32; // type synonym

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}