use serdev::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
struct User {
    name: String,
    age:  usize,
}

#[derive(Debug, PartialEq, Deserialize)]
struct VUser {
    name: String,
    age:  usize,
}

fn main() {
    assert_eq!(
        serde_json::to_string(&User {
            name: String::from("serdev"),
            age:  0
        }).unwrap(),
        r#"{"name":"serdev","age":0}"#
    );
    assert_eq!(
        serde_json::from_str::<User>(
            r#"{"age":4,"name":"ohkami"}"#
        ).unwrap(),
        User {
            name: String::from("ohkami"),
            age:  4
        }
    );
}
