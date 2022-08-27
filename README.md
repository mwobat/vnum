# vnum

Create enums with a constant value associated to every variant.

## Features

- Get the values via the `.value()` method or via the
  [From](https://doc.rust-lang.org/stable/std/convert/trait.From.html "docs for std::convert::From")
  / [Into](https://doc.rust-lang.org/stable/std/convert/trait.Into.html "docs for std::convert::Into")
  Traits
- Your attributes (`#[...]`) and documentation are automatically added to the generated enum
- Documentation is generated showing the value of each variant:
  [Image](./assets/generated_docs_example_dark.png)

## Examples
<!-- 
Keep in sync with the examples folder.

Don't add `pub` to these examples,
but do add it to the ones in the examples folder,
otherwise they wouldn't show up in the docs.
-->

```rust
value_enum! {
    enum Fruit: &'static str {
        Apple = "red",
        Banana = "yellow",
        Pear = "green"
    }
}

// Get the value with the `.value()` method:
let apple = Fruit::Apple;
println!("Apple: {}", apple.value()); // Apple: red

// Get the value with the From / Into traits:
let pear = Fruit::Pear;
let value: &str = pear.into();
println!("Pear: {}", value); // Pear: green
```

`Note:` The type of the values must be specified after the enum name, just like above (`&'static str` in this case).

`Note:` If the value type is a reference (`&`) or contains references, the
[`'static`](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/static_lifetime.html)
lifetime must be used,\
otherwise the Rust compiler would not know where the value is borrowed from.

<br>

```rust
value_enum! {
    #[derive(Debug)]
    enum Color: u8 {
        Red = 1,
        Green = 2,
        Yellow = 3
    }
}

let red = Color::Red;
println!("{:?}: {}", red, red.value()); // Red: 1

let yellow = Color::Yellow;
let value: u8 = yellow.into();
println!("Yellow: {}", value); // Yellow: 3
```

`Note:` If you want more traits implemented for your enum, you have to do it yourself.\
In the example above, the
[Debug](https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html "docs for std::fmt::Debug")
trait is derived.

`Note:` Only constant expressions are allowed to the right of the equals sign,\
which means they must be evaluable at compile time.\
Look here for all kinds of constant expressions: <https://doc.rust-lang.org/reference/const_eval.html#constant-expressions>

## Alternatives

- ### **Simple [constants](https://doc.rust-lang.org/reference/items/constant-items.html)**

  Easy, but you can't:
  - limit the possible values
  - add additional items (e.g. methods, trait impl's, constants)

  Example of using simple constants:

  ```rust
  const RED: u8 = 1;
  const GREEN: u8 = 2;
  const YELLOW: u8 = 3;

  fn display_color(color: u8) { }

  display_color(RED);
  display_color(GREEN);

  // But also accepts other `u8` values:
  display_color(42);
  ```

  You could additionally:
  - Create a
  [type alias](https://doc.rust-lang.org/reference/items/type-aliases.html)
  to improve readability:

    ```rust
    type Color = u8;
    // `Color` is now an alias for `u8`
    fn display_color(color: Color) { }

    display_color(RED);

    // Note: Because `Color` is only an alias and not a new type,
    //       you can still use any other `u8` value:
    display_color(42);
    ```

  - Put the constants in an own module to use them like `Color::RED` :

    ```rust
    mod Color {
        const RED: u8 = 1;
        // ...
    }
    ```

  <br>

- ### **Enum with [disciminators](https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations)**

  - Enum
    [disciminators](https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations)
    can only be integers,\
    so you wouldn't be able to recreate the `&str` example from above.\
    You can cast variants to an integer type via `as`.

  Example of using an enum with disciminators:

  ```rust
  enum Color {
      Red = 1,
      Green = 2,
      Yellow = 3
  }

  fn display_color(color: Color) {
    // Now cast to any integer type via `as`:
    takes_u8(color as u8);
    takes_i32(color as i32);
  }

  display_color(Color::Yellow);
  ```

  You could additionally:
  - Create a method to get the value:

    ```rust
    impl Color {
        fn value(&self) -> u8 {
            self as u8
        }
    }
    // ...
    takes_u8(color.value())
    // ...
    ```

  <br>

- ### **Manually convert from enum variant to value**

  This is exactly what this library does automatically.\

  Example of manually converting from enum variant to value:

  ```rust
  enum Color {
      Red,
      Green,
      Yellow
  }

  impl Color {
      const fn value(&self) -> u8 {
          const RED: u8 = 1;
          const GREEN: u8 = 2;
          const YELLOW: u8 = 3;
          match self {
              Color::Red => RED,
              Color::Green => GREEN,
              Color::Yelllow => YELLOW
          }
      }
  }

  display_color(Color::Yellow);

  fn display_color(color: Color) {
    // Now cast to any integer type via `as`:
    takes_u8(color as u8);
    takes_i32(color as i32);
  }
  ```

  `Note:` Apart from generating a method like this, this libarary generates documentation and a
  [From](https://doc.rust-lang.org/stable/std/convert/trait.From.html "docs for std::convert::From")
  implementation.\
  Look at the beginning of the file for more information.

<br>

## Planned features

- Allow creating multiple value enums in one macro invocation
- Option to ensure unique values
- Option to disable automatic documentation generation
- Make the note about the value type in the generated docs clickable\
  (currently doesn't work because rustdoc only creates links for types which are/contain no references)
- [no_std](https://doc.rust-lang.org/stable/reference/names/preludes.html#the-no_std-attribute)
  support
- Maybe:
  - Get variant by value
  - Optional "wildcard" variant which can hold all values of the type
  - Make the names of duplicate values aliases like in pythons
    [enum module](https://docs.python.org/3/library/enum.html)
  - Optional
    [Debug](https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html "docs for std::fmt::Debug")
    implementation which shows the variant name and value, also like in pythons enum module

<br>

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
