#![cfg_attr(all(doc, not(docsrs)), doc = include_str!("../../README.md"))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use ::serde::de::{self, Deserialize, Deserializer};
pub use ::serde::ser::{self, Serialize, Serializer};
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use serdev_derive::{Deserialize, Serialize};
#[cfg(feature = "derive")]
#[doc(hidden)]
pub mod __private__ {
    pub use ::serde;
    pub use serdev_derive::consume;
    pub type DefaultError = ::std::string::String;
    pub fn default_error(e: impl std::fmt::Display) -> DefaultError {
        e.to_string()
    }
}
