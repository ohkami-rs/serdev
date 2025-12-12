<div align="center">
    <h1>SerdeV</h1>
    SerdeV - Serde with Validation
</div>

<br>

- Just a wrapper of <a href="https://github.com/serde-rs/serde" target="_blank">Serde</a> and 100% compatible
- Declarative validation in deserialization by `#[serde(validate = "...")]`

<div align="right">
    <a href="https://github.com/ohkami-rs/serdev/blob/main/LICENSE" target="_blank">
        <img alt="License" src="https://img.shields.io/crates/l/ohkami.svg" />
    </a>
    <a href="https://github.com/ohkami-rs/serdev/actions" target="_blank">
        <img alt="CI status" src="https://github.com/ohkami-rs/serdev/actions/workflows/CI.yml/badge.svg"/>
    </a>
    <a href="https://crates.io/crates/serdev" target="_blank">
        <img alt="crates.io" src="https://img.shields.io/crates/v/serdev" />
    </a>
</div>

## Example ([closure.rs](https://github.com/ohkami-rs/serdev/blob/main/examples/examples/closure.rs))

```toml
[dependencies]
serdev     = "0.3"
serde_json = "1.0"
```

```rust
use serdev::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(validate = "|p| (p.x * p.y <= 100)
    .then_some(())
    .ok_or(\"x * y must not exceed 100\")")]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = serde_json::from_str::<Point>(r#"
        { "x" : 1, "y" : 2 }
    "#).unwrap();

    // Prints point = Point { x: 1, y: 2 }
    println!("point = {point:?}");

    let error = serde_json::from_str::<Point>(r#"
        { "x" : 10, "y" : 20 }
    "#).unwrap_err();

    // Prints error = x * y must not exceed 100
    println!("error = {error}");
}
```

Of course, you can use it in combination with some validation tools like [validator](https://crates.io/crates/validator)!
( working example: [validator.rs](https://github.com/ohkami-rs/serdev/blob/main/examples/examples/validator.rs) )

## Attribute

- `#[serde(validate = "function")]`

  Automatically validate by the `function` in deserialization. The `function` must be an *expression* that is
  callable as type `fn(&self) -> Result<(), impl Display>` (of course the error type must be known at compile time).
  
  (*expression*: name or path to a `fn` or a method, an inlined closure as above, or even a block expression or function calling
  or anything that are finally evaluated as `fn(&self) -> Result<(), impl Display>`)
  
  Errors are converted to a `String` internally and passed to `serde::de::Error::custom`.

- `#[serde(validate(by = "function", error = "Type"))]`

  Using given `Type` for the validation error, without conversion.
  The `function` signature must explicitly return `Result<(), Type>`.
  
  This will be preferred in **no-std** use, or, maybe when you need better performance in error cases.

Both `"function"` and `"Type"` above accept path like `"crate::util::validate"`.

Additionally, `#[serdev(crate = "path::to::serdev")]` is supported for reexport from another crate.

## License

Licensed under MIT LICENSE ( [LICENSE](https://github.com/ohkami-rs/serdev/blob/main/LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT) ).
