mod cli;
mod client;
mod model;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = cli::cmd().get_matches();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(match matches.get_one::<String>("log").unwrap().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let mut client = client::Client::default();
            client.init()?;

            let query = sub_matches.get_one::<String>("QUERY").unwrap();

            let data = client.find_album(query);
        }
        _ => {}
    }

    Ok(())
}
