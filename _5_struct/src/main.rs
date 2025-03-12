#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

struct Color(i32, i32, i32);
struct UnitStruct;

fn build_rect(w: u32, height: u32) -> Rectangle {
    Rectangle {
        width: w,
        height, // works because variable and field called the same
    }
}

fn main() {
    let mut r1 = build_rect(5, 6);
    r1.width = 10;

    let r2 = Rectangle { width: 5, ..r1 };

    let black = Color(0, 0, 0);
    let unit = UnitStruct;

    println!("{r1:?}"); // works because of #[derive(Debug)]
    println!("{}", r1.area());
    println!("{:?}", Rectangle::square(5));
}
