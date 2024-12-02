use anyhow::Result;
use httpbin::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
  env_logger::init();
  let client = httpbin::Client::new("https://httpbin.org");

  // MARK: /get
  let _ = client.get(None).send().await?;

  // MARK: /image
  let _ = client.image().send().await?;

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
