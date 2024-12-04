use std::{fs::File, io::Write};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::init();
  let client = httpbin::Client::new("https://httpbin.org");

  // MARK: /get
  let _ = client.get(None).send().await?;

  // MARK: /image
  let input = httpbin::ImageInput {
    arg: httpbin::ImageArg::Path(String::from("png")),
  };
  let png_resp = client.image(&input).send().await?;
  let mut png_img = File::create("output.png")?;
  png_img.write_all(&png_resp.bytes.to_vec())?;

  // MARK: /post
  let input = httpbin::PostInput {
    query_arg1: String::from("Hola!"),
    query_arg2: 3,
    header_arg1: String::from("Amigo"),
    header_arg2: 6,
    body_arg1: String::from("Uno"),
  };
  let _ = client.post(Some(&input)).send().await?;
  Ok(())
}
