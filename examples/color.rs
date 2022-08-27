// Keep in sync with the README examples

use vnum::value_enum;

value_enum! {
    /// Your documentation could be here 😀
    #[derive(Debug)]
    pub enum Color: u8 {
        /// Documentation for enum variants is also supported.
        Red = 1,
        Green = 2,
        Yellow = 3
    }
}

fn main() {
    let red = Color::Red;
    println!("{:?}: {}", red, red.value()); // Red: 1

    let yellow = Color::Yellow;
    let value: u8 = yellow.into();
    println!("Yellow: {}", value); // Yellow: 3
}
