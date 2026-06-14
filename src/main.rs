#[tokio::main]
async fn main() -> anyhow::Result<()> {
    metron::run().await
}
