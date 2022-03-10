use clap::StructOpt;
use word_game::{AppResult, run, args::Args};

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::parse();
    run(&args).await
}