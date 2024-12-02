use std::fmt;

use serde::Deserialize;
use thiserror::Error;

// @see https://blog.burntsushi.net/rust-error-handling
// @see https://www.shuttle.dev/blog/2022/06/30/error-handling

#[derive(Error, Debug)]
pub enum Error {
  #[error("HttpBin Error: {0}")]
  HttpBin(#[from] HttpBinError),
  #[error("HTTP Error: {0}")]
  Http(#[from] reqwest::Error),
  #[error("Serde Error: {0}")]
  Serde(#[from] serde_json::Error),
  #[error("Unknown Error")]
  Unknown,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct HttpBinError {
  pub body: HttpBinErrorBody,
  pub status_code: http::StatusCode,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpBinErrorBody {
  pub errors: Option<Vec<serde_json::Value>>,
  pub message: String,
}

impl fmt::Display for HttpBinError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.body.message)?;
    // if let Some(documentation_url) = &self.documentation_url {
    //   write!(f, "\nDocumentation URL: {documentation_url}")?;
    // }
    if let Some(errors) = &self
      .body
      .errors
      .as_ref()
      .filter(|errors| !errors.is_empty())
    {
      write!(f, "\nErrors:")?;
      for error in errors.iter() {
        write!(f, "\n- {error}")?;
      }
    }
    Ok(())
  }
}

impl std::error::Error for HttpBinError {}
