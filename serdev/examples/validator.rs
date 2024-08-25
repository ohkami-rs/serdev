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

fn main() {
    let signupdata = serde_json::from_str::<SignupData>(r#"
        {
            "mail": "serdev@ohkami.rs",
            "site": "https://ohkami.rs",
            "firstName": "serdev",
            "age": 20,
            "height": 0.0
        }
    "#).unwrap();
    assert_eq!(signupdata, SignupData {
        mail: String::from("serdev@ohkami.rs"),
        site: String::from("https://ohkami.rs"),
        first_name: String::from("serdev"),
        age: 20,
        height: 0.0
    });

    let error = serde_json::from_str::<SignupData>(r#"
        {
            "mail": "serdev@ohkami.rs",
            "site": "https://ohkami.rs",
            "firstName": "serdev",
            "age": 0,
            "height": 0.0
        }
    "#).unwrap_err();
    println!("error: {error}");
}
