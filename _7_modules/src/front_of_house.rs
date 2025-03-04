pub mod hosting {
    pub fn add_to_waitlist() {}
    pub fn seat_at_table() {
        add_to_waitlist();
        super::serving::take_order();
        println!("seat_at_table")
    }
}

mod serving {
    pub fn take_order() {
        println!("take_order")
    }
}
