mod cli;
mod client;
mod model;

use chrono::Utc;
use colored::Colorize;
use std::collections::HashMap;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const COLORS: &[(u8, u8, u8)] = &[(88, 129, 87), (163, 177, 138), (218, 215, 205)];

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

            let mut grouped: HashMap<&String, Vec<&model::Album>> = HashMap::new();

            let items = client.find_album(query).await?.results;
            for item in items.iter() {
                if !grouped.contains_key(&item.artist_name) {
                    grouped.insert(&item.artist_name, Vec::new());
                }
            }

            for item in items.iter() {
                grouped.get_mut(&item.artist_name).unwrap().push(item);
            }

            let mut sorted_groups = grouped.iter().collect::<Vec<_>>();
            sorted_groups.sort_by(|a, b| a.0.cmp(b.0));

            for (artist, group) in sorted_groups.iter() {
                println!(
                    "{}",
                    artist.truecolor(COLORS[1].0, COLORS[1].1, COLORS[1].2)
                );

                for album in *group {
                    let release_date: chrono::DateTime<Utc> = album.release_date.into();

                    println!(
                        "    {} {}",
                        release_date
                            .format("%Y")
                            .to_string()
                            .truecolor(COLORS[0].0, COLORS[0].1, COLORS[0].2)
                            .bold(),
                        album
                            .collection_name
                            .truecolor(COLORS[2].0, COLORS[2].1, COLORS[2].2)
                    );
                }
            }
        }
        _ => {}
    }

    Ok(())
}
