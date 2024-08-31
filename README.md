<div align="center">
    <h1>SerdeV</h1>
    SerdeV - Serde with Validation
</div>

<br>

- Just a wrapper of <a href="https://github.com/serde-rs/serde" target="_blank">Serde</a> and 100% compatible
- Automatic validation in deserialization by `#[serde(validate = "...")]`

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

<br>

## Example

```toml
[dependencies]
serdev     = "0.1"
serde_json = "1.0"
```

```rust
use serdev::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(validate = "Self::validate")]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn validate(&self) -> Result<(), impl std::fmt::Display> {
        if self.x * self.y > 100 {
            return Err("x * y must not exceed 100")
        }
        Ok(())
    }
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

Of course, you can use it in combination with some validation tools like <a href="https://crates.io/crates/validator" target="_blank">validator</a>! ( <a href="https://github.com/ohkami-rs/serdev/blob/main/examples/examples/validator.rs" target="_blank">full example</a> )


## Attribute

- `#[serde(validate = "function")]`

  Automatically validate by the `function` in deserialization. The `function` must be callable as `fn(&self) -> Result<(), impl Display>`.\
  Errors are converted to a `String` internally and passed to `serde::de::Error::custom`.

- `#[serde(validate(by = "function", error = "Type"))]`

  Using given `Type` for validation error without internal conversion. The `function` must explicitly return `Result<(), Type>`.\
  This may be preferred when you need better performance _even in error cases_.\
  For **no-std** use, this is the only way supported.

Both `"function"` and `"Type"` accept path like `"crate::utils::validate"`.


## License

Licensed under MIT LICENSE ( [LICENSE](https://github.com/ohkami-rs/serdev/blob/main/LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT) ).
