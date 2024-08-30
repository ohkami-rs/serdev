use serdev::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct User {
    name: String,
    age:  usize,
}

#[derive(Debug, PartialEq, Deserialize)]
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
#[serde(validate(by = "Self::validate", error = "&'static str"))]
struct EUser {
    name: String,
    age:  usize,
}
impl EUser {
    fn validate(&self) -> Result<(), &'static str> {
        if self.name.is_empty() {
            return Err("`name` must not be empty")
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(validate = "Self::validate")]
struct GUser<'n, Name: From<String>+ToString, Age: From<u8>> {
    name:     Name,
    age:      Age,
    nickname: Option<&'n str>
}
impl<'n, Name: From<String>+ToString, Age: From<u8>> GUser<'n, Name, Age> {
    fn validate(&self) -> Result<(), impl std::fmt::Display> {
        if self.name.to_string().is_empty() {
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

    assert_eq!(
        serde_json::from_str::<VUser>(
            r#"{"age":4,"name":"ohkami"}"#
        ).unwrap(),
        VUser {
            name: String::from("ohkami"),
            age:  4
        }
    );
    assert_eq!(
        serde_json::from_str::<VUser>(
            r#"{"age":4,"name":""}"#
        ).unwrap_err().to_string(),
        "`name` must not be empty"
    );

    assert_eq!(
        serde_json::from_str::<EUser>(
            r#"{"age":4,"name":"ohkami"}"#
        ).unwrap(),
        EUser {
            name: String::from("ohkami"),
            age:  4
        }
    );
    assert_eq!(
        serde_json::from_str::<EUser>(
            r#"{"age":4,"name":""}"#
        ).unwrap_err().to_string(),
        "`name` must not be empty"
    );

    assert_eq!(
        serde_json::from_str::<GUser<String, u8>>(
            r#"{"age":4,"name":"ohkami"}"#
        ).unwrap(),
        GUser {
            name:     String::from("ohkami"),
            age:      4,
            nickname: None
        }
    );
    assert_eq!(
        serde_json::from_str::<GUser<String, u8>>(
            r#"{"age":4,"nickname":"wolf","name":"ohkami"}"#
        ).unwrap(),
        GUser {
            name:     String::from("ohkami"),
            age:      4,
            nickname: Some("wolf")
        }
    );
    assert_eq!(
        serde_json::from_str::<GUser<String, u8>>(
            r#"{"age":4,"nickname":"wolf","name":""}"#
        ).unwrap_err().to_string(),
        "`name` must not be empty"
    );
}
