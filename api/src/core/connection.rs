use tokio::net::UnixStream;

pub async fn connect_core() -> Result<UnixStream, anyhow::Error> {
  Ok(UnixStream::connect("/tmp/branch-vault.sock").await?)
}