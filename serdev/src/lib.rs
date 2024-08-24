pub use serdev_derive::{Serialize, Deserialize};
pub use ::serde::ser::{self, Serialize, Serializer};
pub use ::serde::de::{self, Deserialize, Deserializer};

#[doc(hidden)]
pub mod __private__ {
    pub use serdev_derive::consume;
    pub use ::serde;
    pub type DefaultError = ::std::boxed::Box<dyn ::core::fmt::Display>;
    pub fn default_error(e: impl std::fmt::Display + 'static) -> DefaultError {Box::new(e)}
}
