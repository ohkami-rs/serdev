use reexporter::private::serdev::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serdev(crate = "reexporter::private::serdev")]
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
