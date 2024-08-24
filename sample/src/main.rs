use std::intrinsics::transmute_unchecked;

use serdev::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields,)]
#[serde()]
struct User {
    name: String,
    age:  usize,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields,)]
#[serde(validate = "Self::validate")]
struct VUser {
    name: String,
    age:  usize,
}
impl VUser {
    fn validate(&self) -> Result<(), impl std::fmt::Display> {
        if self.name.is_empty() {
            return Err("`name` must not be empty")
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields,)]
#[serde(validate = "Self::validate")]
struct GUser<'n, Name, Age> {
    name:     Name,
    age:      Age,
    nickname: Option<&'n str>
}
impl<'n, Name, Age> GUser<'n, Name, Age> {
    fn validate(&self) -> Result<(), impl std::fmt::Display> {
        if self.name.is_empty() {
            return Err("`name` must not be empty")
        }
        Ok(())
    }
}

fn main() {
    assert_eq!(
        serde_json::to_string(&User {
            name: String::from("serdev"),
            age:  0
        }).unwrap(),
        r#"{"Name":"serdev","Age":0}"#
    );
    assert_eq!(
        serde_json::from_str::<User>(
            r#"{"Age":4,"Name":"ohkami"}"#
        ).unwrap(),
        User {
            name: String::from("ohkami"),
            age:  4
        }
    );

    assert_eq!(
        serde_json::from_str::<VUser>(
            r#"{"Age":4,"Name":"ohkami"}"#
        ).unwrap(),
        VUser {
            name: String::from("ohkami"),
            age:  4
        }
    );
    assert_eq!(
        serde_json::from_str::<VUser>(
            r#"{"Age":4,"Name":""}"#
        ).unwrap_err().to_string(),
        "`name` must not be empty"
    );
}
