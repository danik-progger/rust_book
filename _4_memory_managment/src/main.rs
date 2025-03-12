fn main() {
    // Example of variable on stack
    let n = 1;
    let mut s = String::from("hello");

    // Example of variable on heap
    s.push_str(" ,world!");
    println!("{s}");

    // If variable is on stack it's copied
    let mut x = 1;
    let mut y = x;
    y += 1;
    println!("{x} {y}");

    // If variable is on the heap it's moved
    // Also works if part of collection is on the heap ex: (i32, String)
    let mut s1 = String::from("hello");
    let mut s2 = s1;
    println!("{s1}"); // - will cause error, because s1 is now garbage

    // Function params returns are also moving values

    // --- 3 RULES ---
    // 1. Only 1 owner
    // 2. 1 mutable reference with no immutable
    // 3. Multiple immutable references with no mutable
    // - Reference cannot outlive it's value

    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;

    let s3 = &mut s; // - will cause error
    println!("Strings: {s1}, {s2}");
    // All immutable references are gone
    let s3 = &mut s; // now it won't cause error

    // Slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let hello = &s[..5];
    let world = &s[6..11];
    let world = &s[6..];
}
