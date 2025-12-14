use serdev::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate, Debug, PartialEq)]
#[serde(validate = "Validate::validate")]
struct User<'a> {
    #[garde(ascii, length(min = 3, max = 25))]
    username: &'a str,
    #[garde(length(min = 15))]
    password: &'a str,
}

fn main() {
    let result = serde_json::from_str::<User>(
        r#"{
            "username": "test",
            "password": "not_a_very_good_paddword"
        }"#
    );
    assert_eq!(
        dbg!(result).unwrap(),
        User {
            username: "test",
            password: "not_a_very_good_paddword",
        }
    );
    
    let result = serde_json::from_str::<User>(
        r#"{
            "username": "test",
            "password": "short_password"
        }"#
    );
    assert!(dbg!(result).is_err());
}
