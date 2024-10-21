// By default everything is private
// mods, funcs, fields - everything

pub mod front_of_house;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    println!("eat_at_restaurant")
}
