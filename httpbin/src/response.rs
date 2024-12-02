use crate::error::{Error, HttpBinError, HttpBinErrorBody};
use bytes::Bytes;
use http::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Response<T: for<'de> Deserialize<'de>> {
  _phantom: PhantomData<T>,
  pub bytes: Bytes,
  pub headers: HeaderMap,
  pub content_length: Option<u64>,
}

impl<T: for<'de> Deserialize<'de>> Response<T> {
  pub fn new(bytes: Bytes, headers: HeaderMap, content_length: Option<u64>) -> Self {
    Response {
      _phantom: PhantomData,
      bytes,
      headers,
      content_length,
    }
  }

  pub fn json(&self) -> Result<T, Error> {
    let json = serde_json::from_slice::<T>(&self.bytes)?;
    Ok(json)
  }
}
