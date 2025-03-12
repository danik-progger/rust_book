use std::slice;

static mut COUNTER: u32 = 0; // global variable

fn main() {
    // Access global variable
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);

    // Dereference a raw pointer 
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Call an unsafe function or method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();
        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    // Access or modify a mutable static variable
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }
 
    fn safe_wrapper() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    // The same as above, but with safe Rust
    // unsafe extern "C" {
    //    safe fn abs(input: i32) -> i32; // not a guarantee
    // }

    // fn c_wrapper() {
    //    println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    #[unsafe(no_mangle)] // don't change name of the function
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    // Implement an unsafe trait
    // & Access fields of a union 
    // Can have unsafe fields
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}
