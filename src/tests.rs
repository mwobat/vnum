use crate::value_enum;

#[test]
fn correct_values() {
    {
        make_test_enum!(1);
        let apple: Fruit = Fruit::Apple;
        let banana: Fruit = Fruit::Banana;
        let blueberry: Fruit = Fruit::Blueberry;
        assert_eq!(apple.value(), "red");
        assert_eq!(banana.value(), "yellow");
        assert_eq!(blueberry.value(), "blue");
    }
    // Check for the test enums with i16 type
    for_each!(
        {
            let apple: Fruit = Fruit::Apple;
            let banana: Fruit = Fruit::Banana;
            let blueberry: Fruit = Fruit::Blueberry;
            assert_eq!(apple.value(), -42_i16);
            assert_eq!(banana.value(), 7_i16);
            assert_eq!(blueberry.value(), 13_i16);
        };
        2 3 4 5 6 7 8
    );
}


// Utility macros:

// Execute code for every given test enum number
macro_rules! for_each {
    ( $code:block ; $n:tt $( $rest:tt )* ) => (
        {
            make_test_enum!($n);
            $code
        }
        for_each!( $code; $( $rest )* );
    );
    ( $_:block ; ) => ()
}

use for_each;

// Generated enums for testing.
macro_rules! make_test_enum {
    ( $n:tt $( $rest:tt )+ ) => (
        make_test_enum!($n);
        make_test_enum!($( $rest )+);
    );
    (1) => (
        // str
        value_enum!{
            enum Fruit: &'static str {
                Apple = "red",
                Banana = "yellow",
                Blueberry = "blue"
            }
        }
    );
    (2) => (
        // i16
        value_enum!{
            enum Fruit: i16 {
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    );
    (3) => (
        // pub i16
        value_enum!{
            pub enum Fruit: i16 {
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    );
    (4) => (
        // item docs i16
        value_enum!{
            /// Fruity docs
            enum Fruit: i16 {
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    );
    (5) => (
        // item docs #2 i16
        value_enum!{
            /// Fruity docs
            /// second line
            enum Fruit: i16 {
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    );
    (6) => (
        // variant docs i16
        value_enum!{
            enum Fruit: i16 {
                /// Docs for the apple
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    );
    (7) => (
        // variant docs #2 i16
        value_enum!{
            enum Fruit: i16 {
                /// Docs for the apple
                /// second apple line
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    );
    (8) => (
        // multiple variant docs i16
        value_enum!{
            enum Fruit: i16 {
                /// Docs for the apple
                /// second apple line
                Apple = -42,
                Banana = 7,
                /// Blue
                Blueberry = 13
            }
        }
    );
}

use make_test_enum;
