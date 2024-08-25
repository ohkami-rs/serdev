#![cfg_attr(feature="DEBUG", doc = include_str!("../../README.md"))]

pub use serdev_derive::{Serialize, Deserialize};
pub use ::serde::ser::{self, Serialize, Serializer};
pub use ::serde::de::{self, Deserialize, Deserializer};

#[doc(hidden)]
pub mod __private__ {
    pub use serdev_derive::consume;
    pub use ::serde;
    pub type DefaultError = ::std::string::String;
    pub fn default_error(e: impl std::fmt::Display) -> DefaultError {e.to_string()}
}
