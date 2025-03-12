use std::fmt;
use std::ops::Add;

// Supertraits
trait OutlinePrint: fmt::Display {
    // can be implemented only for structs that implement fmt::Display
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Overload methods for types from other crate
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub trait Iterator {
    type Item; // can be only 1 impl

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// impl of Add
trait _Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot is flying");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard is flying");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human is flying");
    }
}

fn main() {
    let x = Point { x: 1, y: 0 };
    let y = Point { x: 2, y: 3 };
    println!("{:?}", x + y);

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    <Human as Pilot>::fly(&person); // works even if () are empty

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
