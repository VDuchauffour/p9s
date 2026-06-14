use clap::Parser;
use p9s::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Config::parse();
    let config = args.load()?;
    p9s::run(config).await
}
