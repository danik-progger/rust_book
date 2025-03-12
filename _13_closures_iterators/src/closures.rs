// Closure = arrow function
use std::thread;
use std::time::Duration;

fn expensive_calculation(i: u32) -> u32 {
    println!("slowly calculating");
    thread::sleep(Duration::from_secs(2));
    return i;
}

struct Cacher<T, U>
where T: Fn(U) -> U, U: Copy {
    calc: T,
    value: Option<U>
}

impl<T, U> Cacher<T, U>
where T: Fn(U) -> U, U: Copy {
    fn new(calc: T) -> Cacher<T, U> {
        Cacher {
            calc, value: None
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn calculate_workout(n: u32) {
    let mut expensive_closure = Cacher::new(|i| {
        println!("slowly calculating");
        thread::sleep(Duration::from_secs(2));
        return i;
    });

    if n < 10 {
        println!("{}", expensive_closure.value(10));
        println!("{}", expensive_closure.value(10));
    } else if n < 20 {
        println!("{}", expensive_closure.value(20));
    } else {
        println!("{}", -1);
    }
}

fn main() {
    let i = 1;
    println!("{}", expensive_calculation(i));
    calculate_workout(5);

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32|        { x + 1 };
    let add_one_v4 = |x|          x + 1  ;

    add_one_v4(1);

    let x = 5;
    let equals_x = |z: u32| z == x; // can use vars from outer scope
    let equals_x = move |z: u32| z == x; // this moves x insight closure
}
