// Keep in sync with the README examples

use value_enum::value_enum;

value_enum! {
    pub enum Color: u8 {
        Red = 1,
        Green = 2,
        Yellow = 3
    }
}

fn main() {
    // Get the value with the `.value()` method:
    let red = Color::Red;
    println!("Red: {}", red.value()); // Red: 1

    // Get the value with the From / Into traits:
    let yellow = Color::Yellow;
    let value: u8 = yellow.into();
    println!("Yellow: {}", value); // Yellow: 3
}
