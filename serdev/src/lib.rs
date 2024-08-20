pub use serdev_derive::{Serialize, Deserialize};
pub use ::serde::ser::{self, Serialize, Serializer};
pub use ::serde::de::{self, Deserialize, Deserializer};

#[doc(hidden)]
pub mod __private__ {
    pub use serdev_derive::consume;
    pub use ::serde;
}
