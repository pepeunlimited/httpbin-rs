use crate::serde_utils::as_i64;
use serde::Deserialize;

// MARK: /get

#[derive(Deserialize, Debug)]
pub struct Get {
  pub origin: String,
  pub args: GetArgs,
  pub headers: GetHeaders,
  // #[serde(flatten)]
  // pub other: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct GetArgs {
  #[serde(default)]
  pub query_arg1: String,
  #[serde(default, deserialize_with = "as_i64")]
  pub query_arg2: i64,
}

#[derive(Deserialize, Debug)]
pub struct GetHeaders {
  #[serde(rename = "Accept")]
  pub header_arg1: String,
  // #[serde(rename = "X-Custom-Value", default, deserialize_with = "as_i64")]
  // pub header_arg2: Option<i32>,
  #[serde(rename = "X-Custom-Value", default, deserialize_with = "as_i64")]
  pub header_arg2: i64,
}

#[derive(Clone)]
pub struct GetInput {
  pub query_arg1: String,
  pub query_arg2: i64,
  pub header_arg1: String,
  pub header_arg2: i64,
}

// MARK: /post

#[derive(Deserialize, Debug)]
pub struct Post {
  pub origin: String,
  #[serde(flatten)]
  pub other: std::collections::HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {

  use crate::{http_methods::GetInput, Client2};
  use std::option::Option::Some;

  const ENDPOINT: &str = "https://httpbin.org";

  #[tokio::test]
  async fn http_methods_get_with_inputs() {
    env_logger::init();
    let client = Client2::new(ENDPOINT);
    let input = GetInput {
      query_arg1: String::from("variable1"),
      query_arg2: 2,
      header_arg1: String::from("variable3"),
      header_arg2: 4,
    };
    let r = client.get(Some(&input)).send().await.unwrap();
    match r.json() {
      Ok(get) => {
        assert_eq!(input.query_arg1, get.args.query_arg1);
        assert_eq!(input.query_arg2, get.args.query_arg2);
        assert_eq!(input.header_arg1, get.headers.header_arg1);
        assert_eq!(input.header_arg2, get.headers.header_arg2);
      }
      Err(_) => {
        assert!(false, "should be ok")
      }
    }
  }

  #[tokio::test]
  async fn http_methods_get_without_inputs() {
    let client = Client2::new(ENDPOINT);
    let r = client.get(None).send().await.unwrap();
    match r.json() {
      Ok(get) => {
        assert_eq!("", get.args.query_arg1);
        assert_eq!(0, get.args.query_arg2);
        assert_eq!("*/*", get.headers.header_arg1);
        assert_eq!(0, get.headers.header_arg2);
      }
      Err(_) => {
        assert!(false, "should be ok")
      }
    }
  }
}
