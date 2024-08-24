<div align="center">
    <h1>SerdeV</h1>
    SerdeV - <a href="https://github.com/serde-rs/serde" target="_blank">Serde</a> with Validation
</div>

<br>

- Just a wrapper of serde and 100% serde(derive) compatible
- Validation on deserializing with `#[serde(validate = "function")]`

## Example

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
    let p = serde_json::from_str::<Point>(
        r#"{"x":1,"y":2}"#
    );

    // Prints p = Ok(Point { x: 1, y: 2 })
    println!("p = {:?}", p);

    let p = serde_json::from_str::<Point>(
        r#"{"x":-10,"y":2}"#
    );

    // Prints p = Err(Error("x and y must not be negative", line: 0, column: 0))
    println!("p = {:?}", p);
}
```

## Attribute

- `#[serde(validate = "function")]`

  Perform validation by the function just after deserializing finished. The function must be callable as `fn(&self) -> Result<(), impl Display>`.
Errors are converted to `String` internally and passed to `serde::de::Error::custom`.

- `#[serde(validate(by = "function", error = "Type"))]`

  Use given `Type` for validation error instead of `String`. This will be preferred when you need performance even for error cases.

Both `"function"` and `"Type"` accept path like `"crate::utils::validate"`.

## License

Licensed under MIT LICENSE ( [LICENSE](https://github.com/ohkami-rs/serdev/blob/main/LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT) ).