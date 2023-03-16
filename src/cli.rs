use clap::{arg, crate_version, Command};

pub fn cmd() -> Command {
    Command::new("cli")
        .about("CLI demo app")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .version(crate_version!())
        .subcommand(
            Command::new("search")
                .arg(arg!(<QUERY>).help("Album search query").required(true))
                .about("Search for an album"),
        )
        .arg(
            arg!(--log <LEVEL>)
                .short('l')
                .help("Override log level")
                .long_help("Override log level. Valid values are: trace, debug, info, warn, error")
                .number_of_values(1)
                .default_value("info"),
        )
}
