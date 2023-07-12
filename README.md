# vnum

[![Crates.io](https://img.shields.io/crates/v/vnum.svg)](https://crates.io/crates/vnum)\
assign constant values to enum variants

## Features

- get values via `.value()` or the [From]/[Into] traits
- attributes (`#[...]`) and documentation are automatically adopted
- auto-generated documentation showing the value of each variant:
  [example](./misc/generated_docs_example.png)

[From]: https://doc.rust-lang.org/stable/std/convert/trait.From.html "std::convert::From"
[Into]: https://doc.rust-lang.org/stable/std/convert/trait.Into.html "std::convert::Into"

## Example
<!--
Keep in sync with the examples folder.
Don't add `pub` to these examples,
but do add it to the ones in the examples folder,
otherwise they wont show up in the docs.
-->

```rust
use vnum::value_enum;
value_enum! {
    enum Fruit: &'static str {
        Apple = "red",
        Banana = "yellow",
        Pear = "green"
    }
}

let apple = Fruit::Apple;
println!("Apple: {}", apple.value()); // Apple: red

// get value via From/Into traits
let pear = Fruit::Pear;
let value: &str = pear.into();
println!("Pear: {}", value); // Pear: green
```

### Note

- the value type must be specified (`&'static str` in the example)
- only [constant expressions] can be used to the right of the equals sign
- if the type is a reference or contains references, the [`'static`] lifetime must be used

[constant expressions]: https://doc.rust-lang.org/reference/const_eval.html#constant-expressions
[`'static`]: https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/static_lifetime.html

## Planned features

- create multiple value enums with one macro invocation
- optionally ensuring unique values
- option to specify whether documentation should be generated
- [no_std](https://doc.rust-lang.org/stable/reference/names/preludes.html#the-no_std-attribute)
  support
- possibly:
  - get variant by unique value
  - optional "wildcard" variant; can hold all values of the type
  - optional
    [Display]
    impl showing variant name and value, e.g. `<Fruit::Pear "green">`
  - clickable value type in generated documentation

[Display]: https://doc.rust-lang.org/stable/std/fmt/trait.Display.html "std::fmt::Debug"

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed, without any additional terms or conditions.

### License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
