use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Image {
  // NOTICE: do not decode the image because the response is actually binary
}

pub struct ImageInput {
  pub arg: ImageArg,
}

pub enum ImageArg {
  Accept(String),
  Path(String),
}

#[cfg(test)]
mod tests {

  use crate::{http_methods::GetInput, images::ImageArg, Client, ImageInput};
  use std::{fs::File, io::Write, option::Option::Some};

  const ENDPOINT: &str = "https://httpbin.org";

  #[tokio::test]
  async fn images_with_accept_input() {
    let input = ImageInput {
      arg: ImageArg::Accept(String::from("image/jpeg")),
    };

    let client = Client::new(ENDPOINT);
    let resp = client.image(&input).send().await.unwrap();
    let mut file = File::create("output.jpeg").unwrap();
    file.write_all(&resp.bytes.to_vec()).unwrap();
  }

  #[tokio::test]
  async fn images_with_accept_path() {
    let input = ImageInput {
      arg: ImageArg::Path(String::from("png")),
    };

    let client = Client::new(ENDPOINT);
    let resp = client.image(&input).send().await.unwrap();
    let mut file = File::create("output.png").unwrap();
    file.write_all(&resp.bytes.to_vec()).unwrap();
  }
}
