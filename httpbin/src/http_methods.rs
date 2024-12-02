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
  pub args: PostArgs,
  pub headers: PostHeaders,
  #[serde(default)]
  pub json: Option<PostJson>,
}

#[derive(Clone)]
pub struct PostInput {
  pub query_arg1: String,
  pub query_arg2: i64,
  pub header_arg1: String,
  pub header_arg2: i64,
  pub body_arg1: String,
}

#[derive(Deserialize, Debug)]
pub struct PostArgs {
  #[serde(default)]
  pub query_arg1: String,
  #[serde(default, deserialize_with = "as_i64")]
  pub query_arg2: i64,
}

#[derive(Deserialize, Debug)]
pub struct PostHeaders {
  #[serde(rename = "Accept")]
  pub header_arg1: String,
  // #[serde(rename = "X-Custom-Value", default, deserialize_with = "as_i64")]
  // pub header_arg2: Option<i32>,
  #[serde(rename = "X-Custom-Value", default, deserialize_with = "as_i64")]
  pub header_arg2: i64,
}

#[derive(Deserialize, Debug)]
pub struct PostJson {
  #[serde(default)]
  pub body_arg1: String,
  // #[serde(default, deserialize_with = "as_i64")]
  // pub query_arg2: i64,
}

#[cfg(test)]
mod tests {

  use crate::{http_methods::GetInput, Client, PostInput};
  use std::option::Option::Some;

  const ENDPOINT: &str = "https://httpbin.org";

  // MARK: /get

  #[tokio::test]
  async fn http_methods_get_with_inputs() {
    env_logger::init();
    let client = Client::new(ENDPOINT);
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
    let client = Client::new(ENDPOINT);
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

  // MARK: /post

  #[tokio::test]
  async fn http_methods_post_without_inputs() {
    let client = Client::new(ENDPOINT);
    let r = client.post(None).send().await.unwrap();
    match r.json() {
      Ok(post) => {
        assert_eq!("", post.args.query_arg1);
        assert_eq!(0, post.args.query_arg2);
        assert_eq!("*/*", post.headers.header_arg1);
        assert_eq!(0, post.headers.header_arg2);
      }
      Err(_) => {
        assert!(false, "should be ok")
      }
    }
  }

  #[tokio::test]
  async fn http_methods_post_with_inputs() {
    let input = PostInput {
      query_arg1: String::from("variable1"),
      query_arg2: 2,
      header_arg1: String::from("variable3"),
      header_arg2: 4,
      body_arg1: String::from("body_arg1"),
    };
    let client = Client::new(ENDPOINT);
    let r = client.post(Some(&input)).send().await.unwrap();
    match r.json() {
      Ok(post) => {
        assert_eq!(input.query_arg1, post.args.query_arg1);
        assert_eq!(input.query_arg2, post.args.query_arg2);
        assert_eq!(input.header_arg1, post.headers.header_arg1);
        assert_eq!(input.header_arg2, post.headers.header_arg2);
        if let Some(json) = post.json {
          assert_eq!(input.body_arg1, json.body_arg1);
        } else {
          assert!(false, "should be ok")
        }
      }
      Err(_) => {
        assert!(false, "should be ok")
      }
    }
  }
}
