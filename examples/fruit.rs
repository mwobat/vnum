// Keep in sync with the README examples

use value_enum::value_enum;

value_enum! {
    pub enum Fruit: &'static str {
        Apple = "red",
        Banana = "yellow",
        Pear = "green"
    }
}

fn main() {
    // Get the value with the `.value()` method:
    let apple = Fruit::Apple;
    println!("Apple: {}", apple.value()); // Apple: red

    // Get the value with the From / Into traits:
    let pear = Fruit::Pear;
    let value: &str = pear.into();
    println!("Pear: {}", value); // Pear: green
}
