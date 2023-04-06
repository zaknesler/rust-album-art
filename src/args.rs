use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "art", version, author, about)]
pub struct Cli {
    /// Override log level
    ///
    /// [valid options: trace, debug, info, warn, error]
    #[clap(
        long,
        short = 'l',
        number_of_values = 1,
        default_value = "info",
        value_name = "LEVEL"
    )]
    pub log: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Search for an album
    Search {
        /// Album search query
        #[clap(required = true)]
        query: String,
    },
}
