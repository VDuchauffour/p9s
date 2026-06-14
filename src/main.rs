use clap::Parser;
use p9s::config::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let config = args.load()?;
    p9s::run(config).await
}
