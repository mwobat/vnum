// Keep in sync with the README examples

use value_enum::value_enum;

value_enum! {
    /// Your documentation could be here 😀
    #[derive(Debug)]
    pub enum Fruit: &'static str {
        /// Documentation for enum variants is also supported.
        Apple = "red",
        Banana = "yellow",
        Pear = "green"
    }
}

fn main() {
    let apple = Fruit::Apple;
    println!("{:?}: {}", apple, apple.value()); // Apple: red

    let pear = Fruit::Pear;
    let value: &str = pear.into();
    println!("Pear: {}", value); // Pear: green
}
