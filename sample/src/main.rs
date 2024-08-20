use serdev::{Serialize, Deserialize};

#[derive(Serialize)]
struct User {
    name: String,
    age:  usize,
}

fn main() {}
