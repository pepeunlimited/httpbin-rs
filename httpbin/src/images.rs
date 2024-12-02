use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Image {
  // NOTICE: do not decode the image because the response is actually binary
}

#[cfg(test)]
mod tests {

  use crate::{http_methods::GetInput, Client};
  use std::{fs::File, io::Write, option::Option::Some};

  const ENDPOINT: &str = "https://httpbin.org";

  #[tokio::test]
  async fn http_methods_get_with_inputs() {
    let client = Client::new(ENDPOINT);
    let resp = client.image().send().await.unwrap();
    let mut file = File::create("output.jpeg").unwrap();
    file.write_all(&resp.bytes.to_vec()).unwrap();
  }
}
