<div align="center">
    <h1>SerdeV</h1>
    SerdeV - Serde with Validation
</div>

<br>

- Just a wrapper of [Serde](https://github.com/serde-rs/serde) (100% compatible),
- implementing `serde::{Serialize, Deserialize}` for your structs,
- with providing `#[serde(validate = "...")]` for declarative validation in `#[derive(Deserialize)]`.

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

<details>
<summary>Why <code>serdev</code>? Do you know <a href="https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate">"Parse, don't validate"</a>?</summary>

A manual implementation for "Parse, don't validate" without `serdev` will be like:

```rust,ignore
#[derive(serde::Deserialize)]
struct Point {
    x: i32,
    y: i32
}

#[derive(serde::Deserialize)]
#[serde(try_from = "Point")]
struct ValidPoint(Point);

impl TryFrom<Point> for ValidPoint {
    // ...
}
```

This is, actually, (almost) exactly what `serdev` does!

Such manual implementation may be a trigger of mistakes like using `Point` directly for parsing user's input.
`serdev` eliminates such kind of mistakes, automatically performing the specified validation.

---

Or, manual `Deserialize` impl?:

```rust,ignore
struct Point {
    x: i32,
    y: i32
}

impl<'de> serde::Deserialize<'de> for Point {
    // ...
}
```

Indeed this doesn't cause such misuses, but produces boilerplate... (more and more boilerplate in complex situation)

---

`#[serde(validate)]` makes, for a struct having complicated conditions, its `Deserialize` itself the valid `parse`r of the struct,
in a clean way with near-zero boilerplate.

If you have no pain on this, you may not need `serdev`.

</details>

## [Example](https://github.com/ohkami-rs/serdev/blob/main/examples/examples/readme.rs)

```toml
[dependencies]
serdev     = { version = "0.3", features = ["derive"] }
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

`#[serde(validate = "...")]` works with:

- other validation tools like [`validator` crate](https://crates.io/crates/validator) or something similar.
  (working example: [validator.rs](https://github.com/ohkami-rs/serdev/blob/main/examples/examples/validator.rs))
  
  ```rust
  use serdev::Deserialize;
  use validator::{Validate, ValidationError};

  #[derive(Deserialize, Debug, PartialEq, Validate)]
  #[serde(validate = "Validate::validate")]
  struct SignupData {
      #[validate(email)]
      mail: String,
      #[validate(url)]
      site: String,
      #[validate(length(min = 1), custom(function = "validate_unique_username"))]
      #[serde(rename = "firstName")]
      first_name: String,
      #[validate(range(min = 18, max = 20))]
      age: u32,
      #[validate(range(min = 0.0, max = 100.0))]
      height: f32,
  }
  
  fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
      if username == "xXxShad0wxXx" {
          // the value of the username will automatically be added later
          return Err(ValidationError::new("terrible_username"));
      }
  
      Ok(())
  }
  ```
  
- inlined closure like `|p| if p.x * p.y <= 100 {Ok(())} else {Err("...")}`, not only a method path.
  (working example: [closure.rs](https://github.com/ohkami-rs/serdev/blob/main/examples/examples/closure.rs))
  
  ```rust
  use serdev::{Serialize, Deserialize};

  #[derive(Serialize, Deserialize, Debug)]
  #[serde(validate = r#"|p| (p.x * p.y <= 100).then_some(()).ok_or("x * y must not exceed 100")"#)]
  struct Point {
      x: i32,
      y: i32,
  }
  ```

## Attribute

- `#[serde(validate = "function")]`

  Automatically validate the deserialized struct by the `function`. The `function` must be an *expression* that is
  callable as type `fn(&self) -> Result<(), impl Display>` (of course the error type must be known at compile time).
  
  (*expression*: an inlined closure as above, or name/path to a `fn` or a method, or even a block expression or function calling
  or anything that are finally evaluated as `fn(&self) -> Result<(), impl Display>`)
  
  Errors are internally converted to a `String` and passed to `serde::de::Error::custom`.

- `#[serde(validate(by = "function", error = "Type"))]`

  Using given `Type` for the validation error, without conversion.
  The `function` signature must be `fn(&self) -> Result<(), Type>`.
  
  This will be preferred in **no-std** use, or, maybe when you need better performance in error cases.

Both `"function"` and `"Type"` above accept path e.g. `"Self::validate"` or `"crate::util::validate"`.

Additionally, `#[serdev(crate = "path::to::serdev")]` is supported for reexport from another crate.

## License

Licensed under MIT LICENSE ( [LICENSE](https://github.com/ohkami-rs/serdev/blob/main/LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT) ).
