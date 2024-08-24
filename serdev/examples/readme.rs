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
