#![allow(dead_code)]

mod args;
mod client;
mod model;
mod results;

use crate::results::GroupedResults;
use clap::Parser;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const COLORS: &[(u8, u8, u8)] = &[(88, 129, 87), (163, 177, 138), (218, 215, 205)];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = args::Cli::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(args.log.parse::<Level>().unwrap_or(Level::INFO))
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    match args.command {
        args::Command::Search { query } => {
            let mut client = client::Client::default();
            client.init()?;

            let res = client.find_album(&query).await?;
            let results = GroupedResults::from(res);

            println!("{}", results);
        }
    }

    Ok(())
}
