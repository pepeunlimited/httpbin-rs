use std::borrow::Borrow;

use crate::{
  http_methods::{Get, GetInput},
  images::{Image, ImageInput},
  request::Request,
  Error, Post, PostInput,
};
use http::{header::ACCEPT, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use reqwest::Client as ReqwestClient;
use serde_json::{json, Value};
use url::Url;

pub struct Client {
  reqwest_client: ReqwestClient,
  base_url: Url,
}

impl Client {
  /// Returns new HttpBin client
  pub fn new(base_url: &str) -> Client {
    // let mut headers = HeaderMap::new();
    // headers.insert(ACCEPT, HeaderValue::from_static("plain/text"));
    // let client = reqwest::Client::builder().build();
    let client = reqwest::Client::builder()
      .connection_verbose(true)
      .use_rustls_tls()
      .build()
      .unwrap();
    let base_url = Url::parse(base_url).unwrap();
    return Client {
      reqwest_client: client,
      base_url,
    };
  }

  // MARK: HTTP Methods -> Testing different HTTP verbs

  /// */get* GET parameters.
  pub fn get(&self, input: Option<&GetInput>) -> Request<Get> {
    let mut req_url = self.base_url.clone();
    req_url.set_path("get");

    let mut headers = HeaderMap::new();
    // set the input values
    if let Some(i) = input {
      req_url.set_query(Some(
        format!("query_arg1={}&query_arg2={}", i.query_arg1, i.query_arg2).as_str(),
      ));
      headers.insert(
        ACCEPT,
        HeaderValue::from_str(i.header_arg1.as_str()).unwrap(),
      );
      headers.insert(
        "X-Custom-Value",
        HeaderValue::from_str(&i.header_arg2.to_string()).unwrap(),
      );
      // headers.insert("X-Custom-Value", HeaderValue::from_static("ASDDSA"));
    }

    Request::<Get>::new(&self.reqwest_client, req_url, headers, Method::GET, None)
  }

  /// */post* POST parameters.
  pub fn post(&self, input: Option<&PostInput>) -> Request<Post> {
    let mut req_url = self.base_url.clone();
    req_url.set_path("post");

    let mut headers = HeaderMap::new();
    let mut parameters: Option<Value> = None;

    // set the input values
    if let Some(i) = input {
      req_url.set_query(Some(
        format!("query_arg1={}&query_arg2={}", i.query_arg1, i.query_arg2).as_str(),
      ));
      headers.insert(
        ACCEPT,
        HeaderValue::from_str(i.header_arg1.as_str()).unwrap(),
      );
      headers.insert(
        "X-Custom-Value",
        HeaderValue::from_str(&i.header_arg2.to_string()).unwrap(),
      );
      parameters = Some(json!({
        "body_arg1": i.body_arg1
      }));
    }

    Request::<Post>::new(
      &self.reqwest_client,
      req_url,
      headers,
      Method::POST,
      parameters,
    )
  }

  /// */put* PUT parameters.
  pub fn put(&self) {
    todo!();
  }

  /// */delete* DELETE parameters.
  pub fn delete(&self) {
    todo!();
  }

  /// */patch* PATCH parameters.
  pub fn patch(&self) {
    todo!();
  }

  // MARK: Request inspection -> Inspect the request data

  /// */ip* returns the requester's IP Address.
  pub fn ip(&self) {
    todo!();
  }

  // MARK: Images -> Returns different image formats

  /// */image* returns different image formats
  pub fn image(&self, input: &ImageInput) -> Request<Image> {
    let mut req_url = self.base_url.clone();
    let mut headers = HeaderMap::new();

    // set the input values
    match &input.arg {
      crate::images::ImageArg::Accept(v) => {
        req_url.set_path("image");
        headers.insert(ACCEPT, HeaderValue::from_str(v.as_str()).unwrap());
      }
      crate::images::ImageArg::Path(v) => {
        req_url.set_path(&format!("image/{}", v.as_str()));
      }
    }

    Request::<Image>::new(&self.reqwest_client, req_url, headers, Method::GET, None)
  }
}
