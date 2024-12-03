pub mod client;
pub mod error;
pub mod http_methods;
pub mod images;
pub mod request;
pub mod response;
pub mod serde_utils;

pub use client::Client;
pub use error::{Error, HttpBinError, HttpBinErrorBody};
pub use http_methods::{Get, GetInput, Post, PostInput};
pub use images::{ImageArg, ImageInput};
pub use request::Request;
pub use response::Response;
pub use serde_utils::as_i64;
pub use Error::{Http, HttpBin, Serde, Unknown};
