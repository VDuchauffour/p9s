use clap::Parser;
use metron::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Config::parse();
    let config = args.load()?;
    metron::run(config).await
}
