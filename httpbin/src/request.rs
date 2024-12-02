use http::{header::ACCEPT, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use reqwest::Client;
use serde::Deserialize;
use std::marker::PhantomData;
use url::Url;

use crate::{
  error::{Error, HttpBinError, HttpBinErrorBody},
  Response2,
};

#[derive(Debug)]
pub struct Request2<'a, T: for<'de> Deserialize<'de>> {
  _phantom: PhantomData<T>,
  reqwest_client: &'a Client,
  url: Url,
  headers: HeaderMap,
  method: Method,
}

impl<'a, T: for<'de> Deserialize<'de>> Request2<'a, T> {
  pub fn new(client: &'a Client, url: Url, headers: HeaderMap, method: Method) -> Self {
    Request2 {
      _phantom: PhantomData,
      reqwest_client: client,
      url,
      headers,
      method,
    }
  }

  pub async fn send(&self) -> Result<Response2<T>, Error> {
    let resp = self
      .reqwest_client
      .request(self.method.clone(), self.url.as_str())
      .headers(self.headers.clone())
      .send()
      .await?;

    if resp.status().is_success() {
      let content_length = resp.content_length();
      let headers = resp.headers().clone();
      let bytes = resp.bytes().await?;
      // let json = resp.json::<T>().await?;
      Ok(Response2::new(bytes, headers, content_length))
    } else {
      let status_code = resp.status();
      let bytes = resp.bytes().await?;
      // NOTICE: this error_body could also be generic if wanted
      let error_body: HttpBinErrorBody = serde_json::from_slice(&bytes)?;
      Err(Error::HttpBin(HttpBinError {
        body: error_body,
        status_code,
      }))
    }
  }
}
