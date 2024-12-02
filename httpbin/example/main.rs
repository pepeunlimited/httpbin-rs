use anyhow::Result;
use httpbin::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
  env_logger::init();
  let client = httpbin::Client::new("https://httpbin.org");
  let _ = client.get(None).send().await?;
  let _ = client.image().send().await?;
  Ok(())
}
