#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tokio::spawn(async {
    eighx_ui::window::launch().unwrap();
  })
  .await?;

  Ok(())
}
