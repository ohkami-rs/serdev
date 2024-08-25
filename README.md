<div align="center">
    <h1>SerdeV</h1>
    SerdeV - <a href="https://github.com/serde-rs/serde" target="_blank">Serde</a> with Validation
</div>

<br>

- Just a wrapper of serde_derive and 100% compatible
- Validation on deserializing with `#[serde(validate = "...")]`

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
serdev = "0.1"
```

```rust
use serdev::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(validate = "Self::validate")]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn validate(&self) -> Result<(), impl std::fmt::Display> {
        if self.x < 0 || self.y < 0 {
            return Err("x and y must not be negative")
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
        { "x" : -10, "y" : 2 }
    "#).unwrap_err();

    // Prints error = x and y must not be negative
    println!("error = {error}");
}
```


## Attribute

- `#[serde(validate = "function")]`

  Perform validation by the function just after deserializing finished. The function must be callable as `fn(&self) -> Result<(), impl Display>`.\
  Currently, errors are converted to `String` internally and passed to `serde::de::Error::custom`.

- `#[serde(validate(by = "function", error = "Type"))]`

  Use given `Type` for validation error without conversion. The function must explicitly return `Result<(), Type>`.\
  This may be preferred when you need better performance _even in error cases_.\
  For **no-std** use, this is the only way supported.

Both `"function"` and `"Type"` accept path like `"crate::utils::validate"`.


## License

Licensed under MIT LICENSE ( [LICENSE](https://github.com/ohkami-rs/serdev/blob/main/LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT) ).